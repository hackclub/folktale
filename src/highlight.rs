const KEYWORDS: &[&str] = &[
    "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif",
    "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda",
    "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield",
];
const CONSTANTS: &[&str] = &["True", "False", "None"];
const BUILTINS: &[&str] = &[
    "abs",
    "bool",
    "dict",
    "enumerate",
    "float",
    "input",
    "int",
    "len",
    "list",
    "max",
    "min",
    "open",
    "print",
    "range",
    "reversed",
    "round",
    "set",
    "sorted",
    "str",
    "sum",
    "tuple",
    "type",
    "zip",
];

pub fn highlight_python(code: &str) -> String {
    let chars: Vec<char> = code.chars().collect();
    let mut out = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let start = i;

        if c == '#' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            push_span(&mut out, "hl-comment", &chars[start..i]);
        } else if c == '"' || c == '\'' {
            i += 1;
            while i < chars.len() && chars[i] != '\n' {
                if chars[i] == '\\' {
                    i += 2;
                    continue;
                }
                let quote = chars[i] == c;
                i += 1;
                if quote {
                    break;
                }
            }
            i = i.min(chars.len());
            push_span(&mut out, "hl-string", &chars[start..i]);
        } else if c.is_ascii_digit() {
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                i += 1;
            }
            push_span(&mut out, "hl-number", &chars[start..i]);
        } else if c.is_alphabetic() || c == '_' {
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let class = if KEYWORDS.contains(&word.as_str()) {
                Some("hl-keyword")
            } else if CONSTANTS.contains(&word.as_str()) {
                Some("hl-constant")
            } else if BUILTINS.contains(&word.as_str()) {
                Some("hl-builtin")
            } else {
                None
            };
            match class {
                Some(class) => push_span(&mut out, class, &chars[start..i]),
                None => push_escaped(&mut out, &chars[start..i]),
            }
        } else {
            push_escaped(&mut out, &chars[start..=i]);
            i += 1;
        }
    }

    out.replace("\t", "  ")
}

fn push_span(out: &mut String, class: &str, chars: &[char]) {
    out.push_str("<span class=\"");
    out.push_str(class);
    out.push_str("\">");
    push_escaped(out, chars);
    out.push_str("</span>");
}

fn push_escaped(out: &mut String, chars: &[char]) {
    for &c in chars {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
}
