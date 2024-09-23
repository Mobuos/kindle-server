use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn nav() -> Markup {
    html! {
        header {
            .logo {
                h1 { "Kindle Server" }
            }
            nav {
                ul {
                    li hx-get="/stats/battery" hx-trigger="load, click, every 3m" {
                        "Checking Battery.."
                    }
                    li #filecount hx-get="/stats/files" hx-trigger="load, click, updateImage from:body" {
                        "Checking File Count.."
                    }
                    li hx-post="/sync" hx-swap="none" hx-indicator=".sync" {
                        button {
                            div .sync #sync-text { "Sync" }
                            img .sync #sync-loading width="16px" src="/static/resources/pulse-rings-2.svg";
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
            (PreEscaped("<script src=\"https://unpkg.com/htmx.org@1.9.12\"
            integrity=\"sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2\" 
            crossorigin=\"anonymous\"></script>"))
            link rel="stylesheet" href="/static/style.css";
        }
        body {
            (nav())
            .content {(content)}
            footer {
                // TODO
            }
        }
    }
}

pub fn show_image(image_name: &str) -> Markup {
    html! {
        form {
            input type="hidden" name="text" value=(image_name);
            img .image src={"converted/"(image_name)}
                onerror="this.onerror=null; this.src='static/resources/notfound.png'"
                hx-post="/set"
                hx-vals={"{{\"image_name\": "(image_name)"}}"}
                hx-trigger="click";
            p { (image_name) }
            button .delete hx-delete={"/"(image_name)} hx-target="closest form" hx-swap="outerHTML swap:0.5s" { "Delete" }
        }
    }
}
