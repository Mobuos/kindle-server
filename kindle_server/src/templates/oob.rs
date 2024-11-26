use maud::{html, Markup};

use crate::templates::elements;

// OOB = Out of Band
// Check https://htmx.org/attributes/hx-swap-oob/

// TODO: Send everything everytime?
// Just appending the image at the end could show a different order upon reload
// I believe `ls` uses alphabetical order :/
// Maybe we will need to do this some other way in the future, if I add something like
// "pinned items", but we will cross that bridge when we come to it, for now on my browser
// it seems to recognize the repeat images and just caches them, so no big problem for now
pub fn swap_server_images(server_images: Option<&Vec<String>>) -> Markup {
    html! {
        #server-images hx-swap-oob="innerHTML" {
            (elements::server_images(server_images))
        }
        (self::force_update_file_count())
    }
}

pub fn force_update_file_count() -> Markup {
    html! {
        li #filecount hx-swap-oob="outerHTML" hx-get="/stats/files" hx-trigger="load, click, updateImage from:body"
            ."text-white/70" {
            "Checking File Count.."
        }
    }
}

pub fn error_banner(title: &str, message: &str) -> Markup {
    html! {
        div hx-swap-oob="outerHTML:#newalert" {
            .alert.transition-opacity.duration-300.bg-red-100.border.border-red-400.text-red-700.px-4.py-3.rounded.relative.mb-6 role="alert" {
                strong .font-bold { (title)": " }
                span .block."sm:inline" { (message) }
                span .absolute.top-0.bottom-0.right-0.px-4.py-3 {
                    button .fill-current.h-6.w-6.text-red-500
                    hx-on="click: this.closest('.alert').remove()"
                    {
                        svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" {
                            title { "Close" }
                            path d="M14.348 14.849a1.2 1.2 0 0 1-1.697 0L10 11.819l-2.651 3.029a1.2 1.2 0 1 1-1.697-1.697l2.758-3.15-2.759-3.152a1.2 1.2 0 1 1 1.697-1.697L10 8.183l2.651-3.031a1.2 1.2 0 1 1 1.697 1.697l-2.758 3.152 2.758 3.15a1.2 1.2 0 0 1 0 1.698z" {}
                        }
                    }
                }
            }
            #newalert {}
        }
    }
}
