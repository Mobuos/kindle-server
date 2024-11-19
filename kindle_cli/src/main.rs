use std::{path::PathBuf, process};

use clap::{Parser, Subcommand};
use kindle_manager::KindleManager;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    address: String,

    #[arg(short, long, default_value_t = String::from("/mnt/us/images"))]
    location: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Prepares the Kindle by disabling the screensaver and other services.
    Prep,
    /// Lists all files in the specified location
    List,
    /// Deletes a file
    Delete { filename: String },
    /// Pushes a file to the specified location
    Push {
        file_path: PathBuf,
        filename: String,
    },
    /// Pulls a file from the specified location
    Pull {
        filename: String,
        file_path: PathBuf,
    },
    /// Shows an image on screen from the specified location
    Set { filename: String },
    /// Prints current battery percentage
    BatteryInfo,
    /// Shows a debug message on screen
    DebugPrint { message: String },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let kindle_manager = match KindleManager::new(args.address, args.location).await {
        Ok(manager) => manager,
        Err(err) => {
            eprintln!("Failed to create a session with the provided address.");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    match args.command {
        Commands::Prep => prep(&kindle_manager).await,
        Commands::List => list_files(&kindle_manager).await,
        Commands::Delete { filename } => delete_file(&kindle_manager, &filename).await,
        Commands::Push {
            file_path,
            filename,
        } => push_file(&kindle_manager, &file_path, &filename).await,
        Commands::Pull {
            filename,
            file_path,
        } => pull_file(&kindle_manager, &filename, &file_path).await,
        Commands::Set { filename } => set_image(&kindle_manager, &filename).await,
        Commands::BatteryInfo => info_battery(&kindle_manager).await,
        Commands::DebugPrint { message } => debug_print(&kindle_manager, &message).await,
    }
}

async fn prep(kindle_manager: &KindleManager) {
    match kindle_manager.prep().await {
        Ok(_) => println!("Kindle is prepared to show images now"),
        Err(err) => {
            eprintln!("Failed to prepare the Kindle. Restart it before trying again");
            eprintln!("{err}");
            process::exit(1);
        }
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

async fn pull_file(kindle_manager: &KindleManager, filename: &str, file_path: &PathBuf) {
    match kindle_manager.pull_file(filename, file_path).await {
        Ok(_) => println!("Pulled \"{filename}\""),
        Err(err) => {
            eprintln!("Failed to pull file");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

async fn push_file(kindle_manager: &KindleManager, file_path: &PathBuf, filename: &str) {
    match kindle_manager.push_file(file_path, filename).await {
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
        Ok(_) => println!("Printed \"{text}\""),
        Err(err) => {
            eprintln!("Failed to print debug message!");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
