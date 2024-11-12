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
    let km = KindleManager::new(args.address, args.location)
        .await
        .expect("Failed to create KindleManager");

    if let Ok(files) = km.get_files().await {
        for file in files {
            println!("{}", file);
        }
    }
}
