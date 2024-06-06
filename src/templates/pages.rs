use maud::{html, Markup};

use super::elements::base;

pub fn main() -> Markup {
    let content = html! {
        h1 { "Olá mundo" }

        form hx-post="/" hx-encoding="multipart/form-data" {
            label for="filename" {"pick a file name:"} br;
            input type="text" name="filename" placeholder="File name" required;
            br; br;
            label for="file" {"Choose a file"} br;
            input type="file" id="file" name="file" accept="image/png, image/jpeg";
            br; br;
            button type="submit" {"Submit"}
        }
    };
    base("Main", content)
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
    base(title, content)
}
