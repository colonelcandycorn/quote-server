use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use entity::quote::Model as QuoteModel;
use entity::quote::Entity as QuoteEntity;
use service::data_access::DataAccess;
use serde::Deserialize;
use serde_json::json;

use axum::{
    Json,
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, get_service, post},
    Router,
};

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

fn read_quotes_from_file(file_path: &str) -> Result<Vec<QuoteModel>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let quotes: Vec<QuoteModel> = serde_json::from_reader(reader)?;
    Ok(quotes)
}

#[derive(Clone)]
struct AppState {
    db_conn: DatabaseConnection
}

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    page_size: Option<u64>
}

#[axum::debug_handler]
async fn list_quotes(
    state: State<AppState>,
    Query(params): Query<Params>
) -> Result<impl IntoResponse, StatusCode> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_quotes_in_page(&state.db_conn, page, page_size).await {
        Ok((quotes, num_pages)) => {
            Ok(Json(json!({
                "quotes": quotes,
                "next_page": page + 1,
                "num_pages": num_pages, // this is way off for some reason?
            }
            )))
        },
        Err(e) => {
            Err(
                StatusCode::INTERNAL_SERVER_ERROR
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize the database connection
    let db = Database::connect(format!("sqlite:{}", args.db_path)).await?;

    // Initialize the database if the --init flag is provided
    if args.init {
        // need to open ../static/assets/quotes.json and parse into entity::quote::Model
        // then call DataAccess::create_quote(db, quote).await
        // to create the quote in the database

        let quotes = read_quotes_from_file("./static/assets/quotes.json")?;
        for quote in quotes {
            let active_model = DataAccess::create_quote(&db, quote).await?;
            println!("Created quote: {:?}", active_model);
        }
    }

    let state = AppState { db_conn: db };

    let app = Router::new()
        .route("/quotes", get(list_quotes))
        .with_state(state);
        // .route("/quotes/:id", get())
        // .route("/quotes", post())
        //.layer(Extension(db)


    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .await?;

    Ok(())
}
