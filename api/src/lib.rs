pub mod json;
pub mod template;

use sea_orm::DatabaseConnection;
use tower_http::trace;

use axum::routing::{get, Router};

#[derive(Clone)]
pub struct AppState {
    db_conn: DatabaseConnection,
}

impl AppState {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        AppState { db_conn }
    }
}

pub fn template_router(state: AppState) -> Router<()> {
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    Router::new()
        .route("/", get(template::get_root))
        .route("/quotes", get(template::get_quotes))
        .route("/tags/{tag_id}", get(template::get_tag_and_associated_quotes))
        .route("/authors/{author_id}", get(template::get_author_and_associated_quotes))
        .route("/authors", get(template::get_authors))
        .route("/tags", get(template::get_tags))
        .layer(trace_layer)
        .with_state(state)
}
