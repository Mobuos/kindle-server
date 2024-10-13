use maud::{html, Markup};

use super::elements;

pub fn main(server_images: &Vec<String>) -> Markup {
    let content = html! {
        .mx-auto.max-w-5xl.px-4.py-12 {
            form hx-post="/" hx-encoding="multipart/form-data" hx-swap="none"
                .grid.grid-cols-1.gap-x-6.pb-12 {
                    // Choose image and rename
                    div {
                        label for="file" .block.text-sm.font-medium.leading-6.text-gray-900
                            { "Choose an image:" }
                        .mt-2.flex.max-w-md.rounded-md.shadow-sm.ring-1.ring-inset.ring-gray-300.bg-white
                        ."focus-within:ring-inset"."focus-within:ring-indigo-600"."focus-within:ring-2" {
                            // Browse button
                            label for="file" .cursor-pointer.py-3.px-4.bg-indigo-600.rounded-l-md.text-white.text-sm
                                ."hover:bg-indigo-500"
                                { "Browse..." }
                            // Image name text input
                            input type="text" id="filename" name="filename" placeholder="Image name"
                                oninput="this.userChanged=true"
                                .block.flex-1.border-0.bg-transparent.text-gray-900.text-sm.font-semibold
                                ."placeholder:text-gray-400"."focus:ring-0";
                        }
                        input type="file" id="file" name="file" accept="image/png, image/jpeg" required
                            onchange="set_filename_from_upload()"
                            .text-sm.text-gray-500
                            ."file:hidden"."focus:outline-none";
                    }

                    // Kindle Orientation
                    div {
                        // TODO
                    }

                    // Image adjustments
                    div {
                        // TODO
                    }

                    // Background Color
                    div {
                        // TODO
                    }

                    // Submit buttons
                    .flex.items-center.justify-end.gap-x-6 {
                        button name="set_image" value="false" type="submit"
                            hx-vals="{\"teste\": false}"
                            .border-2.border-gray-300.rounded-md.px-3.py-2.text-sm.text-indigo-700.font-semibold.shadow-sm
                            ."hover:border-indigo-400"."hover:bg-indigo-100"
                            { "Upload" }
                        button name="set_image" value="true" type="submit"
                            hx-vals="{\"teste\": true}"
                            .rounded-md.bg-indigo-600.px-3.py-2.text-sm.font-semibold.leading-6.text-white.shadow-sm
                            ."hover:bg-indigo-500"."focus-visible:outline"."focus-visible:outline-2"
                            { "Upload and Set" }
                    }
            }

            // Separator
            .border-b."border-gray-900/10" {}

            #server-images {
                @if server_images.is_empty() {
                    p { "No images found on the Kindle!" }
                } @else {
                    @for filename in server_images {
                        (elements::show_image(filename))
                    }
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
        li #filecount hx-swap-oob="outerHTML" hx-get="/stats/files" hx-trigger="load, click, updateImage from:body"
            ."text-white/70" {
            "Checking File Count.."
        }
    }
}
