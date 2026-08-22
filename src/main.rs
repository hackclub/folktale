use std::collections::HashMap;
use std::fs;
use std::io;

use actix_files::Files;
use actix_web::middleware::DefaultHeaders;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::header, routes, web};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};

mod highlight;

use highlight::highlight_python;

const UI_DIR: &str = "ui";
const GUIDE_DIR: &str = "guide";
const TEMPLATE_DIR: &str = "templates";
const PAGES: u32 = 7;
const PLACEHOLDER: &str = "{GUIDE}";
const PROGRESS_PLACEHOLDER: &str = "{PROGRESSBAR}";
const NAV_PLACEHOLDER: &str = "{NAV}";
const NAVIGATOR_PLACEHOLDER: &str = "{NAVIGATOR}";

struct Guide {
    pages: HashMap<u32, String>,
}

fn render_guide() -> io::Result<Guide> {
    let template = fs::read_to_string(format!("{TEMPLATE_DIR}/guide.html"))?;

    let mut sources: HashMap<u32, String> = HashMap::new();
    for entry in fs::read_dir(GUIDE_DIR)? {
        let path = entry?.path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(number) = name.split('-').next().and_then(|n| n.parse::<u32>().ok()) else {
            continue;
        };
        sources.insert(number, fs::read_to_string(&path)?);
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let mut titles: Vec<(u32, String)> = Vec::new();
    for number in 1..=PAGES {
        let mut title = format!("Page {number}");
        if let Some(markdown) = sources.get(&number) {
            let mut parser = Parser::new_ext(markdown, options);
            while let Some(event) = parser.next() {
                if matches!(
                    event,
                    Event::Start(Tag::Heading {
                        level: HeadingLevel::H1,
                        ..
                    })
                ) {
                    let mut text = String::new();
                    for event in parser.by_ref() {
                        match event {
                            Event::Text(t) | Event::Code(t) => text.push_str(&t),
                            Event::End(TagEnd::Heading(_)) => break,
                            _ => {}
                        }
                    }
                    title = text;
                    break;
                }
            }
        }
        let title = title
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        titles.push((number, title));
    }

    let mut pages = HashMap::new();
    for number in 1..=PAGES {
        let markdown = sources.get(&number).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("no markdown file for guide page {number}"),
            )
        })?;

        let mut events = Vec::new();
        let mut python: Option<String> = None;
        for event in Parser::new_ext(markdown, options) {
            match event {
                Event::Start(Tag::Image { .. }) => {
                    events.push(Event::Html(
                        "<span class=\"guide-image\">".to_string().into(),
                    ));
                    events.push(event);
                }
                Event::Start(Tag::Link { ref dest_url, .. }) => {
                    let href = dest_url
                        .replace('&', "&amp;")
                        .replace('"', "&quot;")
                        .replace('<', "&lt;")
                        .replace('>', "&gt;");
                    events.push(Event::Html(
                        format!(
                            "<a href=\"{href}\" target=\"_blank\" rel=\"noopener noreferrer\">"
                        )
                        .into(),
                    ));
                }
                Event::End(TagEnd::Image) => {
                    events.push(event);
                    events.push(Event::Html("</span>".into()));
                }
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang)))
                    if lang.as_ref() == "python" =>
                {
                    python = Some(String::new());
                }
                Event::Text(ref text) if python.is_some() => {
                    python.as_mut().expect("python block open").push_str(text);
                }
                Event::End(TagEnd::CodeBlock) if python.is_some() => {
                    let code = python.take().expect("python block open");
                    events.push(Event::Html(
                        format!(
                            "<pre><code class=\"language-python\">{}</code></pre>",
                            highlight_python(&code)
                        )
                        .into(),
                    ));
                }
                other => events.push(other),
            }
        }

        let mut body = String::new();
        html::push_html(&mut body, events.into_iter());

        let progress = f64::from(number) / f64::from(PAGES) * 100.0;
        let progress_bar = format!(
            "<div id=\"progress-bar\"><div id=\"progress-bar-fill\" style=\"width: {progress:.2}%\"></div></div>"
        );

        let previous = if number > 1 {
            format!("<a href=\"/guide/{}\">&larr; Previous</a>", number - 1)
        } else {
            String::from("<span></span>")
        };
        let next = if number < PAGES {
            format!("<a href=\"/guide/{}\">Next &rarr;</a>", number + 1)
        } else {
            String::from("<span></span>")
        };
        let nav = format!("<nav id=\"guide-nav\">{previous}{next}</nav>");

        let mut navigator = String::from("<nav id=\"guide-navigator\">");
        for (page, title) in &titles {
            let class = if *page == number {
                " class=\"current\""
            } else {
                ""
            };
            navigator.push_str(&format!("<a href=\"/guide/{page}\"{class}>{title}</a>"));
        }
        navigator.push_str("</nav>");

        pages.insert(
            number,
            template
                .replace(PLACEHOLDER, &body)
                .replace(PROGRESS_PLACEHOLDER, &progress_bar)
                .replace(NAV_PLACEHOLDER, &nav)
                .replace(NAVIGATOR_PLACEHOLDER, &navigator),
        );
    }

    Ok(Guide { pages })
}

#[get("/guide.html")]
async fn guide_html_redirect() -> impl Responder {
    HttpResponse::MovedPermanently()
        .insert_header((header::LOCATION, "/guide/1"))
        .finish()
}

#[routes]
#[get("/guide")]
#[get("/guide/")]
async fn guide_index() -> impl Responder {
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/guide/1"))
        .finish()
}

#[get("/guide/{page}")]
async fn guide_page(page: web::Path<u32>, guide: web::Data<Guide>) -> impl Responder {
    match guide.pages.get(&page) {
        Some(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html.clone()),
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let guide = web::Data::new(render_guide()?);
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(2000);

    println!("Listening on http://0.0.0.0:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(guide.clone())
            .service(guide_html_redirect)
            .service(guide_index)
            .service(Files::new("/guide/images", format!("{GUIDE_DIR}/images")))
            .service(guide_page)
            .service(Files::new("/", UI_DIR).index_file("index.html"))
            .wrap(
                DefaultHeaders::new()
                    .add((header::CROSS_ORIGIN_OPENER_POLICY, "same-origin"))
                    .add((header::CROSS_ORIGIN_EMBEDDER_POLICY, "require-corp"))
                    .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
