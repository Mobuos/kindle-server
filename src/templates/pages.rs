use maud::{html, Markup};

use super::elements;

pub fn main(server_images: &Vec<String>) -> Markup {
    let content = html! {
        form hx-post="/" hx-encoding="multipart/form-data" hx-swap="none" {
            label for="filename" {"File name:"} br;
            input type="text" id="filename" name="filename" placeholder="crab_image"
                  oninput="this.userChanged=true";
            br;
            label for="file" {"Choose a file:"} br;
            input type="file" id="file" name="file" accept="image/png, image/jpeg" required
                  onchange="set_filename_from_upload()";
            br;
            input type="checkbox" id="set_image" name="set_image";
            label for="set_image" {"Set image after upload?"};
            br;
            button type="submit" {"Upload"}
        }

        #server-images {
            @if server_images.is_empty() {
                p { "No images found on the Kindle!" }
            } @else {
                @for filename in server_images {
                    (elements::show_image(filename))
                }
            }
        }
    };
    elements::base("Main", content)
}

// TODO: Send everything everytime?
// Just appending the image at the end could show a different order upon reload
// I believe `ls` uses alphabetical order :/
// Maybe we will need to do this some other way in the future, if I add something like
// "pinned items", but we will cross that bridge when we come to it, for now on my browser
// it seems to recognize the repeat images and just caches them, so no big problem for now
pub fn oob_swap_server_images(server_images: &Vec<String>) -> Markup {
    html! {
        #server-images hx-swap-oob="innerHTML" {
            @for filename in server_images {
                (elements::show_image(filename))
            }
        }
        (oob_force_update_file_count())
    }
}

pub fn oob_force_update_file_count() -> Markup {
    html! {
        li #filecount hx-swap-oob="outerHTML" hx-get="/stats/files" hx-trigger="load, click, updateImage from:body" {
            "Checking File Count.."
        }
    }
}
