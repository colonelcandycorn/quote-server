use askama::Template;
use clap::Parser;
use entity::quote::Model as QuoteModel;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use service::data_access::DataAccess;
use std::fs::File;
use std::io::BufReader;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "quotes.html")]
struct QuotesTemplate {
    quotes: Vec<QuoteModel>,
}

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
    db_conn: DatabaseConnection,
}

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    page_size: Option<u64>,
}

/*
source: https://askama.readthedocs.io/en/stable/frameworks.html
*/
#[derive(Debug, displaydoc::Display, thiserror::Error)]
enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
    /// Had trouble with database
    Database(#[from] sea_orm::DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {}

        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl {};
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}

#[axum::debug_handler]
async fn get_quotes(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_quotes_in_page(&state.db_conn, page, page_size).await {
        Ok((quotes, _)) => {
            let quotes_template = QuotesTemplate { quotes };

            Ok(Html(quotes_template.render()?))
        }
        Err(e) => Err(AppError::Database(e)),
    }
}

async fn get_root() -> Response {
    Redirect::to("/quotes").into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let db = Database::connect(format!("sqlite:{}", args.db_path)).await?;

    if args.init {
        let quotes = read_quotes_from_file("./static/assets/quotes.json")?;
        for quote in quotes {
            let active_model = DataAccess::create_quote(&db, quote).await?;
            println!("Created quote: {:?}", active_model);
        }
    }

    let state = AppState { db_conn: db };

    let app = Router::new()
        .route("/", get(get_root))
        .route("/quotes", get(get_quotes))
        .with_state(state);

    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
