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
pub fn swap_server_images(server_images: &Vec<String>) -> Markup {
    html! {
        #server-images hx-swap-oob="innerHTML" {
            @for filename in server_images {
                (elements::show_image(filename))
            }
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
