use kindle_manager::{image_converter, KindleManager};
use rocket::{form, Request, State};

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::{env, fs, io};

use rocket::form::{Error, Form};
use rocket::fs::{relative, FileName, FileServer, TempFile};
use rocket::http::{ContentType, Status};

use maud::{html, Markup};

// maud templates
mod templates;
use templates::{elements, errors, oob, pages};

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

// KindleManager Connection
#[derive(Debug)]
struct KindleM {
    manager: KindleManager,
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
    #[field(validate = supported_file_types())]
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

/// Updates list of images on main page
async fn oob_swap_server_images(km: &State<KindleM>, session: &openssh::Session) -> Markup {
    match km.manager.list_files(&session).await {
        Ok(image_names) => oob::swap_server_images(Some(&image_names)),
        Err(err) => {
            eprintln!("> Failed to acquire image names");
            eprintln!("{err}");
            oob::swap_server_images(None)
        }
    }
}

// ------- Validation --------- //

fn valid_filename<'v>(filename: &str) -> form::Result<'v, ()> {
    let filename = FileName::new(filename);
    if !filename.is_safe() {
        Err(form::Error::validation("invalid filename"))?;
    }
    Ok(())
}

fn supported_file_types<'v>(file: &TempFile<'_>) -> form::Result<'v, ()> {
    if let Some(file_ct) = file.content_type() {
        if file_ct == &ContentType::PNG
            || file_ct == &ContentType::JPEG
            || file_ct == &ContentType::WEBP
            || file_ct == &ContentType::BMP
        {
            return Ok(());
        }
    }

    let msg = match file.content_type().and_then(|c| c.extension()) {
        Some(a) => format!("invalid file type: .{}, must be PNG, JPEG, BMP or WEBP", a),
        None => format!("file type must be PNG, JPEG, BMP or WEBP"),
    };

    Err(Error::validation(msg))?
}

// ------- Routes ---------- //
#[get("/")]
async fn view_index(km: &State<KindleM>) -> Markup {
    let session = km.manager.new_session().await;
    match session {
        Ok(session) => match km.manager.list_files(&session).await {
            Ok(filenames) => pages::main(Some(&filenames)),
            Err(err) => {
                eprintln!("> Failed to acquire filenames");
                eprintln!("{err}");
                pages::main(None)
            }
        },
        Err(err) => {
            eprintln!("> Failed to create SSH session");
            eprintln!("{err}");
            pages::main(None)
        }
    }
}

#[get("/forms/rename/<image_name>")]
async fn form_rename(image_name: &str) -> Markup {
    elements::show_edit_image_name(image_name)
}

#[patch("/images/<image_name>", data = "<new_name>")]
async fn rename_image(
    km: &State<KindleM>,
    image_name: &str,
    new_name: Form<TextForm>,
) -> Result<Markup, io::Error> {
    let new_name = format!("{}.png", new_name.text);
    let image_name = format!("{}.png", image_name);

    if new_name == image_name {
        println!("No change in image name, not renaming.");
        return Ok(elements::show_image(&image_name));
    }

    println!("Image name is {image_name}, renaming to {new_name}");

    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");

    match km
        .manager
        .rename_file(&session, &format!("{image_name}"), &format!("{new_name}"))
        .await
    {
        Ok(_) => {
            match fs::rename(
                format!("converted/{image_name}"),
                format!("converted/{new_name}"),
            ) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Failed to rename converted/{image_name} to converted/{new_name}");
                    eprintln!("{err}");
                }
            }
            match fs::rename(format!("images/{image_name}"), format!("images/{new_name}")) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Failed to rename images/{image_name} to images/{new_name}");
                    eprintln!("{err}");
                }
            }
            Ok(elements::show_image(&new_name))
        }
        Err(err) => {
            eprintln!("Failed to rename image on the Kindle");
            eprintln!("{err}");
            Ok(elements::show_image(&image_name))
        }
    }
}

#[post("/", data = "<form>")]
async fn submit_image_form(
    mut form: Form<UploadImage<'_>>,
    server_images: &State<ServerImages>,
    km: &State<KindleM>,
) -> Result<Markup, io::Error> {
    // Establish connection to Kindle
    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");

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
    // Also reduce it's size if needed
    let output = Command::new("magick")
        .arg(format!("images/{}", full_filename))
        .args(["-resize", "1516x2048>"])
        .arg(format!("images/{}.png", user_filename))
        .output()
        .expect("Failed to convert image to PNG");
    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        eprintln!("out: {stdout}");
        eprintln!("err: {stderr}");
    }
    fs::remove_file(format!("images/{}", full_filename)).expect(&format!(
        "Failed to delete original file \"{}\"",
        full_filename
    ));
    full_filename = format!("{}.png", user_filename);

    if form.horizontal {
        println!("Submitted image is being rotated by 90 degrees");
        let output = Command::new("magick")
            .arg(format!("images/{}.png", user_filename))
            .args(["-rotate", "90"])
            .arg(format!("images/{}.png", user_filename))
            .output()
            .expect("Failed to convert image to PNG");
        if !output.status.success() {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();

            eprintln!("out: {stdout}");
            eprintln!("err: {stderr}");
        }
    }

    // Get Background Color
    println!("---------------------- {}", form.background_color);
    let bg_color = match form.background_color {
        "white" => "white",
        "light_gray" => "gray60",
        "dark_gray" => "gray20",
        "black" => "black",
        _ => "white",
    };
    println!("------------- matched: {}", bg_color);

    // TODO: Allow changing background fill - Enum for background
    // Convert image to Kindle-appropriate format
    match image_converter::convert_image(
        &bg_color,
        form.stretch,
        &PathBuf::from(format!("images/{full_filename}")),
        &PathBuf::from(format!("converted/{full_filename}")),
    ) {
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
            return Err(std::io::Error::other("Failed conversion"));
        }
    }

    // Push file to Kindle and set it
    if let Err(err) = km
        .manager
        .push_file(
            &session,
            &PathBuf::from(format!("converted/{}", full_filename)),
            &full_filename,
        )
        .await
    {
        eprintln!("> Failed to push image!");
        eprintln!("{err}");
    }
    if form.set_image {
        if let Err(err) = km.manager.set_image(&session, &full_filename).await {
            eprintln!("> Failed to set image!");
            eprintln!("{err}");
        }
    }

    Ok(oob_swap_server_images(km, &session).await)
}

#[post("/set", data = "<image_name>")]
async fn set_image(image_name: Form<TextForm>, km: &State<KindleM>) -> Status {
    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");
    if let Err(err) = km.manager.set_image(&session, &image_name.text).await {
        eprintln!("> Failed to set image!");
        eprintln!("{err}");
    }
    return Status::Ok;
}

#[post("/sync")]
async fn sync(
    server_images: &State<ServerImages>,
    km: &State<KindleM>,
) -> Result<Markup, io::Error> {
    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");
    match km.manager.list_files(&session).await {
        Ok(image_names) => {
            let kindle_images: HashSet<String> = HashSet::from_iter(image_names);

            // Check for images on the server that aren't on the kindle
            let images = server_images.images.lock().unwrap().clone();
            for s_image in images {
                if !kindle_images.contains(&s_image) {
                    if let Err(err) = km
                        .manager
                        .push_file(
                            &session,
                            Path::new(&format!("converted/{}", s_image)),
                            &s_image,
                        )
                        .await
                    {
                        eprintln!("> Failed to pull file!");
                        eprintln!("{err}")
                    }
                    println!("Missing {} in the kindle", s_image);
                }
            }

            // Check for images on the Kindle that aren't on the server
            for k_image in &kindle_images {
                if !server_images.images.lock().unwrap().contains(k_image) {
                    if let Err(err) = km
                        .manager
                        .pull_file(
                            &session,
                            &k_image,
                            Path::new(&format!("converted/{k_image}")),
                        )
                        .await
                    {
                        eprintln!("> Failed to push file!");
                        eprintln!("{err}")
                    }
                    println!("Missing {} in the server", k_image);
                }
            }
            // Check kindle again for updated images
            Ok(oob_swap_server_images(km, &session).await)
        }
        Err(err) => {
            eprintln!(
                "> Failed to acquire list of images on the Kindle, cancelling the Sync operation"
            );
            eprintln!("{err}");
            Ok(oob::swap_server_images(None))
        }
    }
}

#[delete("/<filename>")]
async fn delete_image(
    filename: &str,
    server_images: &State<ServerImages>,
    km: &State<KindleM>,
) -> Result<Markup, io::Error> {
    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");
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

    if let Err(err) = km.manager.delete_file(&session, filename).await {
        eprintln!("> Failed to delete file!");
        eprintln!("{err}");
    }

    Ok(oob_swap_server_images(km, &session).await)
}

// Route /stats
#[get("/battery")]
async fn stats_battery(km: &State<KindleM>) -> Markup {
    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");
    match km.manager.battery_charge(&session).await {
        Ok(battery) => html! { "Battery: " (battery) "%" },
        Err(err) => {
            eprintln!("> Failed to get battery info");
            eprintln!("{err}");
            html! { "Battery: ??" }
        }
    }
}

#[get("/files")]
async fn stats_files(km: &State<KindleM>) -> Markup {
    let session = km
        .manager
        .new_session()
        .await
        .expect("Failed to create SSH session");
    let count_kindle = match km.manager.list_files(&session).await {
        Ok(images) => format!("{}", images.len()),
        Err(err) => {
            eprintln!("> Failed to get number of files on the Kindle");
            eprintln!("{err}");
            "??".into()
        }
    };
    // let count_kindle = km::get_filenames().len();
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
        .manage(KindleM {
            manager: KindleManager::new("kindle".into(), "/mnt/us/images".into()),
        })
        // Routes
        .mount(
            "/",
            routes![
                submit_image_form,
                view_index,
                set_image,
                delete_image,
                sync,
                form_rename,
                rename_image
            ],
        )
        .mount("/stats", routes![stats_battery, stats_files])
        // Static files
        .mount("/images/", FileServer::from(relative!("../images")))
        .mount("/converted/", FileServer::from(relative!("../converted")))
        .mount("/static/", FileServer::from(relative!("/static/res")))
        .mount("/", FileServer::from(relative!("/static/favicon")).rank(11))
        // Catchers
        .register("/", catchers![not_found])
}
