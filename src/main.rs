mod image_converter_wrapper;
use image_converter_wrapper as ic;
mod kindle_manager_wrapper;
use kindle_manager_wrapper as km;
use rocket::Request;

use std::fs;
use std::path::Path;

use rocket::form::Form;
use rocket::fs::{relative, FileServer, TempFile};
use rocket::http::Status;

use rocket_dyn_templates::{context, tera::Tera, Template};

#[macro_use]
extern crate rocket;

#[derive(Debug, FromForm)]
struct UploadImage<'v> {
    #[field(validate = len(1..=20))]
    filename: &'v str,
    // #[field(validate = supported_images())]
    file: TempFile<'v>,
}

#[get("/hello/<name>")]
fn hello(name: &str) -> Template {
    Template::render(
        "hello",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

fn customize(tera: &mut Tera) {
    tera.add_raw_template(
        "about.html",
        r#"
        {% extends "base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#,
    )
    .expect("valid Tera template");
}

// FIXME: Returning Status directly is not recommended, see https://rocket.rs/guide/v0.5/responses/#responses
//        Just doing this for now because me lazy
// Maybe I will fix this after having more of an idea on what I'm supposed to go, and just go with it for now
#[post("/", data = "<form>")]
async fn submit<'r>(mut form: Form<UploadImage<'r>>) -> Status {
    let filename = form.filename;
    println!("Filename: {}", form.filename);
    match form.file.persist_to(format!("images/{}", filename)).await {
        Ok(_) => (),
        Err(error) => {
            println!("Problem persisting file to system: {:?}", error);
            return Status::InternalServerError;
        }
    };

    // TODO: Allow changing background fill - Enum for background
    match ic::convert(format!("images/{}", filename).as_str(), "gray") {
        Ok(_) => (),
        Err(error) => {
            println!(
                "Problem converting {} to proper kindle-readable format: {:?}",
                filename, error
            );
            return Status::InternalServerError;
        }
    }
    // TODO: Call functions to convert image and transfer it to the kindle
    let converted_path = format!("converted/{}", filename);
    let converted_image = Path::new(converted_path.as_str());
    km::push(converted_image);
    km::set(filename);
    Status::Ok
}

// TODO: for some reason is identifying webp as .bin (?)
// fn supported_images<'v>(file: &TempFile<'_>) -> Result<(), Errors<'v>> {
//     if let Some(file_ct) = file.content_type() {
//         TODO: Doesn't let me use match here (?)
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
    rocket::build()
        .mount("/", routes![submit])
        .mount("/", routes![hello])
        .mount("/", FileServer::from(relative!("/static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
