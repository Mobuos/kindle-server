use maud::{html, Markup, DOCTYPE};

pub fn nav() -> Markup {
    html! {
        header .bg-gray-800.sticky.top-0.z-30 {
            .mx-auto.max-w-7xl.px-6 {
                .relative.flex.h-24.items-center.justify-between {
                    .logo {
                        h1 .text-white.font-bold.text-2xl { "Kindle Server" }
                    }
                    nav {
                        ul .flex.items-center.space-x-8 {
                            li hx-get="/stats/battery" hx-trigger="load, click, every 3m"
                                ."text-white/70" {
                                "Checking Battery.."
                            }
                            li #filecount hx-get="/stats/files" hx-trigger="load, click, updateImage from:body"
                                ."text-white/70" {
                                "Checking File Count.."
                            }
                            li hx-post="/sync" hx-swap="none" hx-indicator=".sync" {
                                button .rounded-md.px-3.py-2.text-white.font-medium."bg-indigo-600"."hover:bg-indigo-500" {
                                    div .sync #sync-text { "Sync" }
                                    img .sync #sync-loading width="16px" src="/static/resources/pulse-rings-2.svg";
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn base(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "KS - " (title)}
            script src="static/helper.js" {}
            script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous" {}
            link rel="stylesheet" href="/static/tw.css";
            link rel="stylesheet" href="https://rsms.me/inter/inter.css";
        }
        body .bg-gray-100.min-h-screen {
            (nav())
            .content {(content)}
            footer {
                // TODO
            }
        }
    }
}

pub fn show_image(filename: &str) -> Markup {
    let image_name = filename.split(".").next().unwrap_or(filename);
    html! {
        form {
            input type="hidden" name="text" value=(filename);
            img .rounded-md src={"converted/"(filename)}
                onerror="this.onerror=null; this.src='static/resources/notfound.png'"
                hx-post="/set"
                hx-vals={"{{\"image_name\": "(filename)"}}"}
                hx-trigger="click";
            p { (image_name) }
            // TODO: Delete should give the image_name without extension, server side deletes all images
            // with this name no matter the extension (Is this dangerous?)
            button hx-delete={"/"(filename)} hx-target="closest form" hx-swap="outerHTML swap:0.5s"
                .border-2.border-gray-300.rounded-md.px-3.py-2.text-sm.text-indigo-700.font-semibold.shadow-sm
                ."hover:border-indigo-400"."hover:bg-indigo-100"
                { "Delete" }
        }
    }
}
