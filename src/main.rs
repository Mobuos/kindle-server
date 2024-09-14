mod image_converter_wrapper;
use image_converter_wrapper as ic;
mod kindle_manager_wrapper;
use kindle_manager_wrapper as km;
use rocket::{Request, State};
use templates::pages::oob_force_update_file_count;

use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;
use std::{fs, io};

use rocket::form::Form;
use rocket::fs::{relative, FileServer, TempFile};
use rocket::http::Status;

use maud::{html, Markup};

// maud templates
mod templates;
use templates::{errors, pages};

#[macro_use]
extern crate rocket;

// Images on the Server
#[derive(Debug)]
struct ServerImages {
    images: Mutex<HashSet<String>>,
}

// Upload Image Form
#[derive(Debug, FromForm)]
struct UploadImage<'v> {
    #[field(validate = len(0..=20))]
    filename: &'v str,
    set_image: bool,
    // #[field(validate = supported_images())]
    file: TempFile<'v>,
}

// Simple text form
#[derive(Debug, FromForm)]
struct TextForm {
    text: String,
}

fn get_server_images() -> impl Iterator<Item = String> {
    fs::read_dir("converted")
        .expect("\"converted\" directory not found!")
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            entry
                .file_name()
                .to_str()
                .expect("Invalid filename")
                .to_owned()
        })
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Markup {
    errors::e404(&req.uri().to_string())
}

// TODO: Organize the code

#[get("/hello/<name>")]
fn hello(name: &str) -> Markup {
    let title = "Hello";
    let items = vec!["One", "Two", "Three"];
    pages::hello(title, name, items)
}

#[get("/")]
fn index() -> Markup {
    let image_names = km::get_image_names();
    pages::main(&image_names)
}

// FIXME: Returning Status directly is not recommended, see https://rocket.rs/guide/v0.5/responses/#responses
//        Just doing this for now because me lazy
// Maybe I will fix this after having more of an idea on what I'm supposed to go, and just go with it for now
#[post("/", data = "<form>")]
async fn submit<'r>(
    mut form: Form<UploadImage<'r>>,
    server_images: &State<ServerImages>,
) -> Result<Markup, io::Error> {
    // Save file to server
    let extension = form
        .file
        .content_type()
        .expect("Failed to get content type")
        .extension()
        .expect("Failed to get extension")
        .to_string();
    let mut filename = form.filename.to_string();
    if form.filename == "" {
        filename = form
            .file
            .name()
            .expect("Empty filename, and failed to get filename from upload")
            .to_string();
    }
    let full_filename = format!("{}.{}", filename, extension);
    match form.file.persist_to(&full_filename).await {
        Ok(_) => (),
        Err(error) => {
            println!("Problem persisting file to system: {:?}", error);
            return Err(error);
        }
    };

    // TODO: Allow changing background fill - Enum for background
    // Convert image
    match ic::convert(&full_filename, "gray") {
        Ok(_) => {
            server_images
                .images
                .lock()
                .unwrap()
                .insert(format!("{}.{}", filename, extension));
        }
        Err(error) => {
            println!(
                "Problem converting {} to proper kindle-readable format: {:?}",
                filename, error
            );
            return Err(error);
        }
    }
    let converted_path = format!("converted/{}.png", filename);
    let converted_image = Path::new(converted_path.as_str());

    // Push file to Kindle and set it
    km::push(converted_image);
    if form.set_image {
        km::set(&full_filename);
    }
    let image_names = km::get_image_names();
    return Ok(pages::oob_swap_server_images(&image_names));
}

#[post("/set", data = "<image_name>")]
async fn set(image_name: Form<TextForm>) -> Status {
    km::set(&image_name.text);
    return Status::Ok;
}

#[post("/sync")]
async fn sync(server_images: &State<ServerImages>) -> Result<Status, io::Error> {
    let kindle_images: HashSet<String> = HashSet::from_iter(km::get_image_names());

    // Check for images on the Kindle that aren't on the server
    for k_image in &kindle_images {
        if !server_images.images.lock().unwrap().contains(k_image) {
            km::pull(k_image, Path::new("converted/"));
            println!("Missing {} in the server", k_image);
        }
    }

    // Check for images on the server that aren't on the kindle
    for s_image in server_images.images.lock().unwrap().iter() {
        if !kindle_images.contains(s_image) {
            km::push(Path::new(&format!("converted/{}", s_image)));
            println!("Missing {} in the kindle", s_image);
        }
    }

    return Ok(Status::Ok);
}

#[delete("/<filename>")]
async fn delete(filename: &str, server_images: &State<ServerImages>) -> Result<Markup, io::Error> {
    match fs::remove_file(format!("converted/{}", filename)) {
        Ok(_) => {
            server_images.images.lock().unwrap().remove(filename);
        }
        Err(error) => {
            println!("Problem removing {}: {:?}", filename, error);
        }
    }

    match fs::remove_file(format!("images/{}", filename)) {
        Ok(_) => (),
        Err(error) => {
            println!("Problem removing {}: {:?}", filename, error);
        }
    }

    km::delete_image(&filename);
    Ok(oob_force_update_file_count())
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

// Route /stats
#[get("/battery")]
async fn stats_battery() -> Markup {
    let battery = km::get_battery();
    html! { "Battery: " (battery) }
}

#[get("/files")]
async fn stats_files() -> Markup {
    let count_kindle = km::get_image_names().len();
    let count_server = fs::read_dir("converted").unwrap().count();
    html! { "Kindle/Server files: " (count_kindle)"/"(count_server)}
}

fn setup() -> std::io::Result<()> {
    // Create necessary dirs
    fs::create_dir_all("images/tmp")?;
    fs::create_dir_all("converted")?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    if let Err(error) = setup() {
        panic!("{error}");
    }
    rocket::build()
        .manage(ServerImages {
            images: Mutex::new(HashSet::from_iter(get_server_images())),
        })
        .mount("/", routes![submit, index, hello, set, delete, sync])
        .mount("/stats", routes![stats_battery, stats_files])
        .mount("/images/", FileServer::from(relative!("/images")))
        .mount("/converted/", FileServer::from(relative!("/converted")))
        .mount("/static/", FileServer::from(relative!("/static")))
        .register("/", catchers![not_found])
}
