use std::{
    path::{self, PathBuf},
    process,
};

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
    push_file(&kindle_manager, "oi.png", "test-images/logo.png").await;

    println!();
    pull_file(&kindle_manager, "oi.png", "test-images/logo.png").await;

    println!();
    list_files(&kindle_manager).await;

    println!();
    delete_file(&kindle_manager, "oi.png").await;

    println!();
    delete_file(&kindle_manager, "oi.png").await;

    println!();
    list_files(&kindle_manager).await;
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
        eprintln!("Failed to delete {}", filename);
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
        Ok(_) => println!("Pulled {}", filename),
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
        Ok(_) => println!("Pushed {}", filename),
        Err(err) => {
            eprintln!("Failed to push file");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
