use std::{path::PathBuf, process};

use clap::Parser;
use kindle_manager::KindleManager;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    address: String,
    #[arg(short, long, default_value_t = String::from("/mnt/us/images"))]
    location: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let kindle_manager = match KindleManager::new(args.address, args.location).await {
        Ok(manager) => manager,
        Err(err) => {
            eprintln!("Failed to create a session with the provided address.");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    println!();
    if let Ok(battery) = kindle_manager.info_battery().await {
        if battery < 30 {
            debug_print(&kindle_manager, "    LOW BATTERY!").await;
        }
        debug_print(&kindle_manager, &format!("{battery}%")).await;
    }
}

async fn list_files(kindle_manager: &KindleManager) {
    match kindle_manager.list_files().await {
        Ok(files) => {
            if files.is_empty() {
                println!("No files found!")
            } else {
                for file in files {
                    println!("- {}", file);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to get files");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

async fn delete_file(kindle_manager: &KindleManager, filename: &str) {
    if let Err(err) = kindle_manager.delete_file(filename).await {
        eprintln!("Failed to delete \"{filename}\"");
        eprintln!("{err}");
        process::exit(1);
    } else {
        println!("Deleted successfully.")
    }
}

async fn pull_file(kindle_manager: &KindleManager, filename: &str, path_file: &str) {
    match kindle_manager
        .pull_file(filename, &PathBuf::from(path_file))
        .await
    {
        Ok(_) => println!("Pulled \"{filename}\""),
        Err(err) => {
            eprintln!("Failed to pull file");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

async fn push_file(kindle_manager: &KindleManager, filename: &str, path_file: &str) {
    match kindle_manager
        .push_file(&PathBuf::from(path_file), filename)
        .await
    {
        Ok(_) => println!("Pushed \"{filename}\""),
        Err(err) => {
            eprintln!("Failed to push file");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

async fn set_image(kindle_manager: &KindleManager, filename: &str) {
    match kindle_manager.set_image(filename).await {
        Ok(_) => println!("Image \"{filename}\" set"),
        Err(err) => {
            eprintln!("Failed to set image \"{filename}\"");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

async fn info_battery(kindle_manager: &KindleManager) {
    match kindle_manager.info_battery().await {
        Ok(battery) => println!("Battery is at {battery}%"),
        Err(err) => {
            eprintln!("Failed to get battery info");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

async fn debug_print(kindle_manager: &KindleManager, text: &str) {
    match kindle_manager.debug_print(text).await {
        Ok(battery) => println!("Printed \"{text}\""),
        Err(err) => {
            eprintln!("Failed to print debug message!");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
