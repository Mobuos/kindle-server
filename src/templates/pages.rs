use maud::{html, Markup};

use super::elements;

pub fn main(server_images: &Vec<String>) -> Markup {
    let content = html! {
        form hx-post="/" hx-encoding="multipart/form-data" hx-swap="none" {
            label for="filename" {"pick a file name, without extension:"} br;
            input type="text" name="filename" placeholder="Leave empty to use uploaded file name";
            br; br;
            label for="file" {"Choose a file"} br;
            input type="file" id="file" name="file" accept="image/png, image/jpeg" required;
            br; br;
            input type="checkbox" id="set_image" name="set_image";
            label for="set_image" {"Upload and set?"};
            br; br;
            button type="submit" {"Submit"}
        }

        #server-images {
            @for image_name in server_images {
                (elements::show_image(image_name))
            }
        }
    };
    elements::base("Main", content)
}

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
    elements::base(title, content)
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
            @for image_name in server_images {
                (elements::show_image(image_name))
            }
        }
    }
}
