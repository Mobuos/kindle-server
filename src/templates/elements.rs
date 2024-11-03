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

pub fn label(content: Markup) -> Markup {
    html! {
        .relative.group.inline-block.w-min {
            .h-2 {}
            span .select-none.text-gray-300 {( icon_info(20, 12) )}
            .opacity-0.invisible."group-hover:opacity-100"."group-hover:visible".absolute.bottom-full."left-1/2".transform."-translate-x-1/2".w-max.z-50 {
                .bg-gray-200.text-gray-900.border.rounded-lg.py-2.z-50.bottom-12 {
                    // Info Contents
                    ( content )
                }
            }
        }
    }
}

pub fn icon_info(width: i32, height: i32) -> Markup {
    html! {
        svg fill="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 416.979 416.979" xml:space="preserve" height=(height) width=(width) {
            g {
                path d="M356.004,61.156c-81.37-81.47-213.377-81.551-294.848-0.182c-81.47,81.371-81.552,213.379-0.181,294.85   c81.369,81.47,213.378,81.551,294.849,0.181C437.293,274.636,437.375,142.626,356.004,61.156z M237.6,340.786   c0,3.217-2.607,5.822-5.822,5.822h-46.576c-3.215,0-5.822-2.605-5.822-5.822V167.885c0-3.217,2.607-5.822,5.822-5.822h46.576   c3.215,0,5.822,2.604,5.822,5.822V340.786z M208.49,137.901c-18.618,0-33.766-15.146-33.766-33.765   c0-18.617,15.147-33.766,33.766-33.766c18.619,0,33.766,15.148,33.766,33.766C242.256,122.755,227.107,137.901,208.49,137.901z" {}
            }
        }
    }
}
