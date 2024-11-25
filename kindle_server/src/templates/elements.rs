use maud::{html, Markup, DOCTYPE};

pub fn nav() -> Markup {
    html! {
        header .bg-gray-800.sticky.top-0.z-30 {
            .mx-auto.max-w-7xl.px-6 {
                .relative.flex.h-24.items-center.justify-between {
                    .logo {
                        h1 .text-white.font-bold.text-2xl { a href="/" { "Kindle Server" } }
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
                                button .btn-primary {
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

pub fn footer() -> Markup {
    html! {
        footer ."bg-gray-800".rounded-lg.shadow.m-4 {
            .w-full.mx-auto.max-w-screen-xl."p-4"."md:flex"."md:items-center"."md:justify-between" {
                p .text-sm.text-gray-400 {
                    "Made by "
                    a href="https://github.com/Mobuos" ."hover:underline" { "mobuos" }
                }
                p .text-sm.text-gray-400 {
                    a href="https://github.com/Mobuos/kindle_server" ."hover:underline" { "Check out the source code!" }
                }
            }
        }
    }
}

pub fn base(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "KS - " (title)}
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0"
            script src="static/helper.js" {}
            script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous" {}
            link rel="stylesheet" href="/static/tw.css";
            link rel="stylesheet" href="https://rsms.me/inter/inter.css";
        }
        body .flex.flex-col.bg-gray-100.h-screen.justify-between {
            (nav())
            .mb-auto {(content)}
            (footer())
        }
    }
}

pub fn server_images(images: Option<&Vec<String>>) -> Markup {
    html! {
        @match images {
            Some(images) if !images.is_empty() => {
                .grid."grid-cols-2"."sm:grid-cols-4"."md:grid-cols-5".gap-x-4.gap-y-5{
                    @for filename in images {
                        (self::show_image(filename))
                    }
                }
            }
            Some(_) => {
                .mx-auto.max-w-screen-sm.text-center {
                    p .mb-4.text-lg.font-light.text-gray-500 { "No images found on the Kindle!" }
                }
            }
            None => {
                .mx-auto.max-w-screen-sm.text-center {
                    p .mb-4.text-lg.font-light.text-gray-500 { "Failed to get images from the Kindle!" }
                }
            }
        }
    }
}

pub fn show_edit_image_name(image_name: &str) -> Markup {
    html! {
        form .flex.items-center.h-10 {
            input type="text" id="text" name="text" value=(image_name)
                .flex-1.text-gray-900.text-sm.font-semibold.w-1.h-full
                .rounded-l-md.shadow-sm.ring-1.ring-inset.border-0.ring-gray-300.bg-white
                ."focus-within:ring-inset"."focus-within:ring-indigo-600"."focus-within:ring-2"
                ."placeholder:text-gray-400";
            button .btn-primary.h-full.rounded-l-none.px-2
                hx-patch={"/images/"(image_name)}
                hx-swap="outerHTML"
                hx-target="closest .image"
                hx-include="previous input" {
                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        polyline points="20 6 9 17 4 12" {}
                    }
            }
        }
    }
}

pub fn show_image_name(image_name: &str) -> Markup {
    html! {
        .flex.items-center.gap-2.h-10 {
            span .text-sm.flex-1 { (image_name) }
            button .btn-secondary.h-full.px-2
                hx-get={"/forms/rename/"(image_name)} hx-target="closest div" hx-swap="outerHTML" {
                svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" {}
                    path d="m15 5 4 4" {}
                }
            }
        }
    }
}

pub fn show_image(filename: &str) -> Markup {
    let image_name = filename.split(".").next().unwrap_or(filename);
    html! {
        form .image {
            input type="hidden" name="text" value=(filename);
            (show_image_name(image_name))
            img .rounded-md.my-2 src={"converted/"(filename)}
                onerror="this.onerror=null; this.src='static/resources/notfound.png'"
                hx-post="/set"
                hx-vals={"{{\"image_name\": "(filename)"}}"}
                hx-trigger="click";
            // TODO: Delete should give the image_name without extension, server side deletes all images
            // with this name no matter the extension (Is this dangerous?)
            .flex.w-full.gap-2 {
                button hx-delete={"/"(filename)} hx-target="closest form" hx-swap="outerHTML swap:0.5s"
                    .btn-secondary.flex-1
                    { "Delete" }
                button hx-post={"/set"} hx-vals={"{{\"image_name\": "(filename)"}}"} hx-swap="none"
                    .btn-primary.flex-1
                    { "Set" }
            }
        }
    }
}

pub fn label(content: Markup) -> Markup {
    html! {
        .relative.group.inline-block.w-min {
            .h-2 {}
            span .select-none.text-gray-300 {( icon_info(20, 12) )}
            .pointer-events-none.opacity-0.invisible.transition-opacity."duration-200"."group-hover:opacity-100"."group-hover:visible".absolute.bottom-full."left-1/2".transform."-translate-x-1/2".w-max.z-50 {
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
