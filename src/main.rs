use clap::Parser;
use sea_orm::Database;
use service::data_access::DataAccess;
use service::data_transfer_objects::{QuoteCreateDTO};
use std::fs::File;
use std::io::BufReader;
use api::template_router;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the database file
    #[arg(short, long, default_value = "quote_server.db")]
    db_path: String,

    /// Whether to initialize the database
    #[arg(short, long)]
    init: bool,
}

fn read_quotes_from_file(
    file_path: &str,
) -> Result<Vec<QuoteCreateDTO>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let quotes: Vec<QuoteCreateDTO> = serde_json::from_reader(reader)?;
    Ok(quotes)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deal with the Arguments
    let args = Args::parse();

    // Deal with Database Connection
    let db = Database::connect(format!("sqlite:{}", args.db_path)).await?;

    if args.init {
        let quotes = read_quotes_from_file("./static/assets/quotes.json")?;
        for quote in quotes {
            let quote_dto = DataAccess::create_quote(&db, quote).await?;
            println!("Created quote: {:?}", quote_dto);
        }
    }

    let state = api::AppState::new(db);

    let app = template_router(state);

    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
