use maud::{html, Markup};

use super::elements::base;

pub fn e404(_uri: &str) -> Markup {
    let content = html! {
        .py-8.px-4.mx-auto.max-w-screen-xl."lg:py-16"."lg:px-6" {
            .mx-auto.max-w-screen-sm.text-center {
                h1 .mb-4.text-7xl.tracking-tight.font-extrabold."lg:text-9xl".text-indigo-600 { "404" }
                p .mb-4.text-3xl.font-bold.text-gray-900 { "There's nothing here :/" }
                p .mb-4.text-lg.font-light.text-gray-500 { "Sorry, no page was found at this address, maybe go back to the home page?" }
                a href="/"
                    .btn-primary
                    { "Back to homepage" }
            }
        }
    };
    base("404", content)
}
