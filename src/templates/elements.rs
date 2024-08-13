use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn nav() -> Markup {
    html! {
        a href="/hello/Unknow" {"Hello"} "|" a href="/about" {"About"}
    }
}

pub fn base(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "KS - " (title)}
            (PreEscaped("<script src=\"https://unpkg.com/htmx.org@1.9.12\"
            integrity=\"sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2\" 
            crossorigin=\"anonymous\"></script>"))
            link rel="stylesheet" href="/static/style.css";
        }
        body {
            (nav())
            (content) br;
            footer {
                a href="/" { "Home" }
            }
        }
    }
}
