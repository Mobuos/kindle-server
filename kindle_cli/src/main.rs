use std::{path::PathBuf, process};

use clap::{Parser, Subcommand, ValueEnum};
use kindle_manager::{image_converter, KindleManager};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Address required to SSH into the Kindle device
    // Default is "kindle", you can add it into your ~/.ssh/config as a host to make things easier :)
    #[arg(short, long, default_value_t = String::from("kindle"))]
    address: String,

    /// Location where files are stored on the Kindle
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
    /// Prints information about the current battery state
    #[clap(visible_aliases = &["battery", "bat"])]
    BatteryInfo,
    /// Shows a debug message on screen
    #[clap(visible_alias = "print")]
    DebugPrint { message: String },
    Backlight {
        #[arg(value_parser = clap::value_parser!(u8))]
        intensity: u8,
    },
    /// Convert an image into a Kindle-appropriate format
    Convert {
        /// Image to be converted
        original_path: PathBuf,
        /// Path to destination
        final_path: PathBuf,
        /// Background color
        #[arg(
            short, 
            long, 
            require_equals = true, 
            num_args = 0..=1, 
            default_value_t = BackgroundColor::Gray, 
            default_missing_value = "Gray", 
            value_enum
        )]
        background: BackgroundColor,
        #[clap(long, short, action)]
        stretch: bool,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum BackgroundColor {
    White,
    LightGray,
    Gray,
    Black,
    // TODO: Automatically find background color based on image
    // Auto,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    
    match args.command {
        Commands::Convert { original_path, 
            final_path, 
            background , 
            stretch} => {
                convert_image(background, stretch, &original_path, &final_path).await;
            },
        command => {
            // For all other commands, initialize KindleManager
            let kindle_manager = match KindleManager::new(args.address, args.location).await {
                Ok(manager) => manager,
                Err(err) => {
                    eprintln!("Failed to create a session with the provided address.");
                    eprintln!("{err}");
                    process::exit(1);
                }
            };
            
            match command {
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
                Commands::Backlight { intensity } => set_backlight(&kindle_manager, intensity).await,
                _ => unreachable!()
            }
        }
    }
}

async fn convert_image(background: BackgroundColor, stretch: bool, origin: &PathBuf, destination: &PathBuf) {
    let color = match background {
        BackgroundColor::White => "white",
        BackgroundColor::LightGray => "gray60",
        BackgroundColor::Gray => "gray20",
        BackgroundColor::Black => "black",
    };

    match image_converter::convert_image(color, stretch, origin, destination) {
        Ok(_) => println!("Converted successfully"),
        Err(err) => {
            eprintln!("Failed to convert the image!");
            eprintln!("{err}");
            process::exit(1);
        }
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
    let charge = match kindle_manager.battery_charge().await {
        Ok(charge) => charge,
        Err(err) => {
            eprintln!("Failed to get battery charge");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let load = match kindle_manager.battery_load().await {
        Ok(load) => load,
        Err(err) => {
            eprintln!("Failed to get battery load");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    println!("Battery is at {charge}% {load}");
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

async fn set_backlight(kindle_manager: &KindleManager, intensity: u8) {
    match kindle_manager.set_backlight(intensity).await {
        Ok(_) => println!("Backlight set at \"{intensity}\""),
        Err(err) => {
            eprintln!("Failed to set backlight intensity!");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
