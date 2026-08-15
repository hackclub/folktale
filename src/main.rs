use actix_files::{Files, NamedFile};
use actix_web::{App, HttpResponse, HttpServer, web};

include!(concat!(env!("OUT_DIR"), "/scripts.rs"));

#[actix_web::get("/guide")]
async fn guide() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./ui/guide.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:8080");

    HttpServer::new(|| {
        let mut app = App::new();
        for &(path, body) in SCRIPTS {
            app = app.route(
                path,
                web::get().to(move || async move {
                    HttpResponse::Ok()
                        .content_type("text/javascript; charset=utf-8")
                        .body(body)
                }),
            );
        }
        app.service(guide)
            .service(Files::new("/", "./ui").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}