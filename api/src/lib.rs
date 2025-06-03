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
        .route(
            "/quotes",
            get(template::get_quotes).post(template::post_quote_form),
        )
        .route(
            "/tags/{tag_id}",
            get(template::get_tag_and_associated_quotes).delete(template::delete_tag),
        )
        .route(
            "/authors/{author_id}",
            get(template::get_author_and_associated_quotes).delete(template::delete_author),
        )
        .route("/authors", get(template::get_authors))
        .route("/tags", get(template::get_tags))
        .route("/submitQuote", get(template::get_quote_form))
        .route(
            "/quotes/{quote_id}",
            get(template::get_single_quote).delete(template::delete_quote),
        )
        .layer(trace_layer)
        .with_state(state)
}

pub fn json_router(state: AppState) -> Router<()> {
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    Router::new()
        .route("/api-docs/openapi.json", get(json::openapi))
        .route("/quotes", get(json::get_quotes).post(json::post_quote))
        .route("/tags", get(json::get_tags))
        .route("/authors", get(json::get_authors))
        .route(
            "/quotes/{quote_id}",
            get(json::get_single_quote)
                //         .put(json::put_single_quote)
                .patch(json::patch_quote_with_new_tag)
                .delete(json::delete_quote),
        )
        .route(
            "/tags/{tag_id}",
            get(json::get_tag_and_associated_quotes)
                //         .put(json::put_single_tag)
                .delete(json::delete_tag),
        )
        .route(
            "/authors/{author_id}",
            get(json::get_author_and_associated_quotes), //         .put(json::put_single_author)
                                                         //         .delete(json::delete_author),
        )
        .layer(trace_layer)
        .with_state(state)
}
