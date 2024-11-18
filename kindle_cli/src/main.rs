use std::process;

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

    match kindle_manager.get_files().await {
        Ok(files) => {
            for file in files {
                println!("- {}", file);
            }
        }
        Err(err) => {
            eprintln!("Failed to get files");
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
