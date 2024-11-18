mod image_converter_wrapper;
use image_converter_wrapper as ic;
mod kindle_manager_wrapper;
use kindle_manager_wrapper as km;
use rocket::{form, Request, State};

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::{env, fs, io};

use rocket::form::Form;
use rocket::fs::{relative, FileName, FileServer, TempFile};
use rocket::http::{ContentType, Status};

use maud::{html, Markup};

// maud templates
mod templates;
use templates::{errors, oob, pages};

#[macro_use]
extern crate rocket;

#[catch(404)]
fn not_found(req: &Request<'_>) -> Markup {
    errors::e404(&req.uri().to_string())
}

// Images on the Server
#[derive(Debug)]
struct ServerImages {
    images: Mutex<HashSet<String>>,
}

// Upload Image Form
#[derive(Debug, FromForm)]
struct UploadImage<'v> {
    #[field(validate = len(0..=20))]
    #[field(validate = valid_filename())]
    filename: &'v str,
    set_image: bool,
    horizontal: bool,
    stretch: bool,
    background_color: &'v str,
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

// ------- Validation --------- //

fn valid_filename<'v>(filename: &str) -> form::Result<'v, ()> {
    let filename = FileName::new(filename);
    if !filename.is_safe() {
        Err(form::Error::validation("invalid filename"))?;
    }
    Ok(())
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

// ------- Routes ---------- //

#[get("/")]
fn view_index() -> Markup {
    let filenames = km::get_filenames();
    pages::main(&filenames)
}

#[post("/", data = "<form>")]
async fn submit_image_form(
    mut form: Form<UploadImage<'_>>,
    server_images: &State<ServerImages>,
) -> Result<Markup, io::Error> {
    // Save file to server
    let og_file_extension = form
        .file
        .content_type()
        .expect("Failed to get content type")
        .extension()
        .expect("Failed to get extension")
        .to_string();

    // Filename should already be checked by form validation, but this guarantees that
    // the filename used is valid.
    // TODO: Check for repeated filenames
    let user_filename = FileName::new(form.filename)
        .as_str()
        .unwrap_or_else(|| {
            form.file
                .name()
                .expect("Invalid filename and failed to get filename from upload")
        })
        .to_string();

    let mut full_filename = format!("{}.{}", user_filename, og_file_extension);
    match form
        .file
        .persist_to(format!("images/{}", full_filename))
        .await
    {
        Ok(_) => (),
        Err(error) => {
            println!("Problem persisting file to system: {:?}", error);
            return Err(error);
        }
    };

    // Convert image to png in the server if it's not a PNG already
    if form.file.content_type() != Some(&ContentType::PNG) {
        println!("Submitted image is being converted into a PNG");
        Command::new("magick")
            .arg(format!("images/{}", full_filename))
            .arg(format!("images/{}.png", user_filename))
            .output()
            .expect("Failed to convert image to PNG");
        fs::remove_file(format!("images/{}", full_filename)).expect(&format!(
            "Failed to delete original file \"{}\"",
            full_filename
        ));
        full_filename = format!("{}.png", user_filename);
    }

    // TODO: Rotation
    if form.horizontal {
        todo!("Horizontal rotation is not implemented yet :(");
    }

    // TODO: Fit / Stretch
    if form.stretch {
        todo!("Stretching the image is not implemented yet :(");
    }

    // Get Background Color
    println!("------------------{}", form.background_color);
    let bg_color = match form.background_color {
        "white" => "white",
        "light_gray" => "gray60",
        "dark_gray" => "gray20",
        "black" => "black",
        _ => "white",
    };
    println!("-------------matched: {}", bg_color);

    // TODO: Allow changing background fill - Enum for background
    // Convert image to Kindle-appropriate format
    match ic::convert(&format!("images/{}", full_filename), &bg_color) {
        Ok(_) => {
            server_images
                .images
                .lock()
                .unwrap()
                .insert(format!("{}", full_filename));
        }
        Err(error) => {
            println!(
                "Problem converting {} to proper kindle-readable format: {:?}",
                user_filename, error
            );
            return Err(error);
        }
    }
    let converted_image = PathBuf::from(format!("converted/{}", full_filename).as_str());

    // Push file to Kindle and set it
    km::push(&converted_image);
    if form.set_image {
        km::set(&full_filename);
    }
    let image_names = km::get_filenames();
    return Ok(oob::swap_server_images(&image_names));
}

#[post("/set", data = "<image_name>")]
async fn set_image(image_name: Form<TextForm>) -> Status {
    km::set(&image_name.text);
    return Status::Ok;
}

#[post("/sync")]
async fn sync(server_images: &State<ServerImages>) -> Result<Markup, io::Error> {
    let image_names = km::get_filenames();
    let kindle_images: HashSet<String> = HashSet::from_iter(image_names);

    // Check for images on the server that aren't on the kindle
    for s_image in server_images.images.lock().unwrap().iter() {
        if !kindle_images.contains(s_image) {
            km::push(Path::new(&format!("converted/{}", s_image)));
            println!("Missing {} in the kindle", s_image);
        }
    }

    // Check for images on the Kindle that aren't on the server
    for k_image in &kindle_images {
        if !server_images.images.lock().unwrap().contains(k_image) {
            km::pull(k_image, Path::new("converted/"));
            println!("Missing {} in the server", k_image);
        }
    }

    // Check kindle again for updated images
    let image_names = km::get_filenames();
    return Ok(oob::swap_server_images(&image_names));
}

#[delete("/<filename>")]
async fn delete_image(
    filename: &str,
    server_images: &State<ServerImages>,
) -> Result<Markup, io::Error> {
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
    Ok(oob::force_update_file_count())
}

// Route /stats
#[get("/battery")]
async fn stats_battery() -> Markup {
    let battery = km::get_battery();
    html! { "Battery: " (battery) }
}

#[get("/files")]
async fn stats_files() -> Markup {
    let count_kindle = km::get_filenames().len();
    let count_server = fs::read_dir("converted").unwrap().count();
    html! { ."text-white/70" { "Kindle/Server files: " (count_kindle)"/"(count_server) }}
}

// ------ Rocket Setup --------- //

fn setup_rocket() -> std::io::Result<()> {
    // Create necessary dirs
    println!(
        "{}",
        env::current_dir()
            .expect("could not get curr dir")
            .to_string_lossy()
    );
    fs::create_dir_all("./images/tmp")?;
    fs::create_dir_all("./converted")?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    if let Err(error) = setup_rocket() {
        panic!("{error}");
    }
    rocket::build()
        // State
        .manage(ServerImages {
            images: Mutex::new(HashSet::from_iter(get_server_images())),
        })
        // Routes
        .mount(
            "/",
            routes![submit_image_form, view_index, set_image, delete_image, sync],
        )
        .mount("/stats", routes![stats_battery, stats_files])
        // Static files
        .mount("/images/", FileServer::from(relative!("../images")))
        .mount("/converted/", FileServer::from(relative!("../converted")))
        .mount("/static/", FileServer::from(relative!("/static")))
        // Catchers
        .register("/", catchers![not_found])
}
