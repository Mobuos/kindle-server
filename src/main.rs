mod image_converter_wrapper;
use image_converter_wrapper as ic;
mod kindle_manager_wrapper;
use kindle_manager_wrapper as km;

use std::fs;

use rocket::form::{Error, Errors, Form};
use rocket::fs::TempFile;
use rocket::http::ContentType;

#[macro_use]
extern crate rocket;

#[derive(Debug, FromForm)]
struct Upload<'v> {
    #[field(validate = len(1..=20))]
    filename: &'v str,
    // #[field(validate = supported_images())]
    file: TempFile<'v>,
}

#[get("/")]
fn index() -> &'static str {
    "
    Hello
    "
}

#[post("/", data = "<form>")]
async fn submit<'r>(mut form: Form<Upload<'r>>) -> std::io::Result<()> {
    let filename = form.filename;
    println!("Filename: {}", form.filename);
    form.file.persist_to(format!("images/{}", filename)).await?;
    // TODO: Allow changing background fill - Enum for background
    ic::convert(format!("images/{}", filename).as_str(), "gray")?;
    // TODO: Call functions to convert image and transfer it to the kindle
    km::push(filename);
    km::set(filename);
    Ok(())
}

// todo: for some reason is identifying webp as .bin (?)
// fn supported_images<'v>(file: &TempFile<'_>) -> Result<(), Errors<'v>> {
//     if let Some(file_ct) = file.content_type() {
//         todo: Doesn't let me use match here (?)
//         if file_ct == &ContentType::PNG
//             || file_ct == &ContentType::JPEG
//             || file_ct == &ContentType::WEBP
//             || file_ct == &ContentType::BMP
//         {
//             return Ok(());
//         }
//     }

//     let msg = match file.content_type().and_then(|c| c.extension()) {
//         Some(a) => format!("invalid file type: .{}, must be PNG, JPEG or WEBP", a),
//         None => format!("file type must be PNG, JPEG or WEBP"),
//     };

//     Err(Error::validation(msg))?
// }

fn setup() -> std::io::Result<()> {
    // Create necessary dirs
    fs::create_dir_all("images/tmp")?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    if let Err(error) = setup() {
        panic!("{error}");
    }
    rocket::build().mount("/", routes![index, submit])
}
