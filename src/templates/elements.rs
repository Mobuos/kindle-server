use maud::{html, Markup, DOCTYPE};

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
            title { "Tera Demo - " (title)}
        }
        body {
            (nav())
            (content)
            footer {
                a href="/" { "Home" }
            }
        }
    }
}
