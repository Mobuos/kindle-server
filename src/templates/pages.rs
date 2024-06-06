use maud::{html, Markup};

use super::elements::base;

pub fn hello(title: &str, name: &str, items: Vec<&str>) -> Markup {
    let content = html! {
        h1 { "Hi " (name) }
        h3 { "Here are your items:" }
        ul {
            @for item in items {
                li { (item) }
            }
        }

        p { "Try going to " a href="/hello/Your%20Name" { "/hello/Your Name"}}
    };
    base(title, content)
}
