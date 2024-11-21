use maud::{html, Markup};

use super::elements;

// Main page, shows submission form, images available on the Kindle and actions available for those.
pub fn main(server_images: Option<&Vec<String>>) -> Markup {
    let content = html! {
        .mx-auto.max-w-5xl.px-4.py-12 {
            // Submission Form
            form hx-post="/" hx-encoding="multipart/form-data" hx-swap="none"
                .grid.grid-cols-1.gap-x-6.gap-y-7.pb-12 {
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
                        label for="horizontal" .block.text-sm.font-medium.leading-6.text-gray-900.w-fit {
                            "Kindle Orientation"
                            ( elements::label( html! {
                                .flex.gap-3.mx-3.my-1 {
                                    .flex.flex-col {
                                        img .object-scale-down.w-20.h-min.m-auto.p-3 src="/static/resources/Kindle-Vertical.png" alt="Image of a Kindle vertically aligned.";
                                        p .text-center.text-gray-900.text-sm.font-semibold { "Vertical" }
                                    }
                                    .flex.flex-col {
                                        img .object-scale-down.w-min.h-20.m-auto.p-3 src="/static/resources/Kindle-Horizontal.png" alt="Image of a Kindle horizontally aligned.";
                                        p .text-center.text-gray-900.text-sm.font-semibold { "Horizontal" }
                                    }
                                }
                            }) )
                        }
                        label for="horizontal" .inline-flex."items-center"."mt-2"."gap-0.5".w-full.max-w-md.rounded-md
                            .cursor-pointer.select-none.bg-white.text-gray-900 {
                            input #horizontal name="horizontal" type="checkbox" value="on" .hidden.peer;
                            span ."w-1/2".text-center.py-1.rounded-l-md.outline.outline-2.outline-indigo-400.text-indigo-700.font-semibold.bg-indigo-100.z-0
                                ."peer-checked:outline-gray-300"."peer-checked:bg-transparent"."peer-checked:text-gray-900"
                                { "Vertical" }
                            span ."w-1/2".text-center.py-1.rounded-r-md.outline.outline-2.outline-gray-300
                                ."peer-checked:outline-indigo-400"."peer-checked:z-0"."peer-checked:text-indigo-700"."peer-checked:font-semibold"."peer-checked:bg-indigo-100"
                                { "Horizontal" }
                        }
                    }

                    // Image adjustment
                    div {
                        label for="stretch" .block.text-sm.font-medium.leading-6.text-gray-900.w-fit {
                            "Image Adjustment"
                            ( elements::label( html! {
                                .flex.gap-3.mx-3.my-1 {
                                    .flex.flex-col {
                                        img .object-scale-down.w-20.h-min.m-auto.p-3 src="/static/resources/Kindle-Fit.png" alt="Image of a Kindle showing an image fitting on its screen, without altering its original resolution.";
                                        p .text-center.text-gray-900.text-sm.font-semibold { "Fit" }
                                    }
                                    .flex.flex-col {
                                        img .object-scale-down.w-20.h-min.m-auto.p-3 src="/static/resources/Kindle-Stretch.png" alt="Image of a Kindle showing an image stretched to fit on the Kindle's original resolution";
                                        p .text-center.text-gray-900.text-sm.font-semibold { "Stretch" }
                                    }
                                }
                            }) )
                        }
                        label for="stretch" .inline-flex."items-center"."mt-2"."gap-0.5".w-full.max-w-md.rounded-md
                            .cursor-pointer.select-none.bg-white.text-gray-900 {
                            input #stretch name="stretch" type="checkbox" .hidden.peer;
                            span ."w-1/2".text-center.py-1.rounded-l-md.outline.outline-2.outline-indigo-400.text-indigo-700.font-semibold.bg-indigo-100.z-0
                                ."peer-checked:outline-gray-300"."peer-checked:bg-transparent"."peer-checked:text-gray-900"
                                { "Fit" }
                            span ."w-1/2".text-center.py-1.rounded-r-md.outline.outline-2.outline-gray-300
                                ."peer-checked:outline-indigo-400"."peer-checked:z-0"."peer-checked:text-indigo-700"."peer-checked:font-semibold"."peer-checked:bg-indigo-100"
                                { "Stretch" }
                        }
                    }

                    // Background Color
                    div {
                        label .block.text-sm.font-medium.leading-6.text-gray-900
                            { "Background Color:" }
                        .flex.gap-6.mt-3 {
                            // Explicit classes for tailwind css generation
                            @let colors = vec![
                                ("bg-white", "text-white", "white"),
                                ("bg-gray-300", "text-gray-300", "light-gray"),
                                ("bg-gray-600", "text-gray-600", "dark_gray"),
                                ("bg-gray-800", "text-gray-800", "black"),
                            ];
                            @for (bg_color, checked_text_color, value) in &colors {
                                input checked name="background_color" value=(value) type="radio"
                                    .cursor-pointer.border-none.w-8.h-8.shadow-sm.(bg_color)
                                    ."checked:bg-none".(checked_text_color)."checked:outline"."checked:outline-4"."checked:outline-indigo-300"
                                    ."checked:outline-offset-4"."focus:outline-none"."focus:outline-offset-4"."focus:outline-indigo-300"
                                    ."focus:outline-4"."focus:ring-2"."focus:ring-offset-8"."focus:ring-indigo-200";
                            }
                        }
                    }

                    // Submit buttons
                    .flex.items-center.justify-end.gap-x-6 {
                        button name="set_image" value="false" type="submit"
                            .btn-secondary
                            { "Upload" }
                        button name="set_image" value="true" type="submit"
                            .btn-primary
                            { "Upload and Set" }
                    }
            }

            // Separator
            .border-b."border-gray-900/10".mb-12 {}

            // Grid of images available on the Kindle
            #server-images {
                (elements::server_images(server_images))
            }
        }
    };
    elements::base("Main", content)
}
