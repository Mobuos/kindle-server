mod image_converter_wrapper;
use image_converter_wrapper as ic;
// mod kindle_manager_wrapper;
// use kindle_manager_wrapper as km;

use std::fs;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::ContentType;

#[macro_use]
extern crate rocket;

#[derive(Debug, FromForm)]
struct Upload<'v> {
    #[field(validate = len(1..=20))]
    filename: &'v str,
    #[field(validate = ext(ContentType::PNG))]
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
    ic::convert(format!("images/{}", filename).as_str(), "black");
    // TODO: Call functions to convert image and transfer it to the kindle
    Ok(())
}

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
