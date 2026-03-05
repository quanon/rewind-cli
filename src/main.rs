use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum)]
enum Collection {
    /// Coming soon (近日入荷)
    ComingSoon,
    /// New items / Restocked (新商品/再入荷)
    NewItems,
}

#[derive(Parser)]
#[command(name = "rewind", about = "Fetch yo-yo product info from YoYo Store Rewind")]
struct Cli {
    /// Collection to fetch
    #[arg(value_enum)]
    collection: Collection,
}

#[derive(Debug, Deserialize)]
struct ShopifyResponse {
    products: Vec<ShopifyProduct>,
}

#[derive(Debug, Deserialize)]
struct ShopifyProduct {
    title: String,
    vendor: String,
    variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
struct Variant {
    price: String,
    #[allow(dead_code)]
    compare_at_price: Option<String>,
}

#[derive(Debug, Serialize)]
struct Product {
    vendor: String,
    title: String,
    price: u64,
}

fn collection_url(collection: &Collection) -> &str {
    match collection {
        Collection::ComingSoon => {
            "https://yoyostorerewind.com/collections/coming-soon/products.json"
        }
        Collection::NewItems => {
            "https://yoyostorerewind.com/collections/new-items/products.json"
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let url = collection_url(&cli.collection);

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; rewind-cli/0.1)")
        .build()?;
    let resp: ShopifyResponse = client.get(url).send()?.json()?;

    let products: Vec<Product> = resp
        .products
        .into_iter()
        .map(|p| {
            let price = p
                .variants
                .first()
                .and_then(|v| v.price.parse::<u64>().ok())
                .unwrap_or(0);

            Product {
                vendor: p.vendor,
                title: p.title,
                price,
            }
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&products)?);
    Ok(())
}
