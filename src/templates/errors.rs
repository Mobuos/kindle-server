use maud::{html, Markup};

use super::elements::base;

pub fn e404(uri: &str) -> Markup {
    let content = html! {
        h1 { "404: Hey! There's nothing here." }
        p { "The page at " (uri) " does not exist!"}
    };
    base("404 - Page not found", content)
}
