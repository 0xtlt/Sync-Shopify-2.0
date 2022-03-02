use crate::shopify::Shopify;
use crate::utils::theme::get_theme_content;
use clap::Parser;

mod shopify;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    stores: String,

    // Shopify theme directory to push
    #[clap(short, long)]
    theme_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    let stores: Vec<&str> = args.stores.split(',').collect();
    let mut shops: Vec<Shopify> = Vec::new();

    // Populate shops
    for store in stores {
        let store_parts = store.split('=').collect::<Vec<&str>>();

        if store_parts.len() != 2 {
            return Err(format!("Invalid store: {}", store));
        }

        let store_name = store_parts[0];
        let store_second_parts = store_parts[1].split(':').collect::<Vec<&str>>();

        if store_second_parts.len() != 2 {
            return Err(format!("Invalid store: {}", store));
        }

        let store_password = store_second_parts[0];
        let store_theme_id = store_second_parts[1].parse::<u64>().unwrap();

        let shopify_store = Shopify {
            store: store_name.to_string(),
            password: store_password.to_string(),
            theme_id: store_theme_id,
        };

        shops.push(shopify_store);
    }

    let local_theme_files = get_theme_content(&args.theme_dir);

    // Save all threads in a variable to wait them all
    let mut threads: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    // For each shop, spawn a new thread
    for shop in shops {
        let shop_clone = shop.clone();
        let theme_files = local_theme_files.clone();

        let thread = tokio::spawn(async move {
            shop_clone.sync(theme_files).await.unwrap();
        });

        threads.push(thread);
    }

    // Wait for all threads to finish
    for thread in threads {
        thread.await.unwrap();
    }

    println!("Done!");

    Ok(())
}
