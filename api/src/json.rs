use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use service::data_access::DataAccess;
use service::data_transfer_objects::{QuoteDTO, QuoteCreateDTO};
use crate::AppState;
use serde_json::json;



#[derive(Deserialize, Serialize)]
pub struct Params {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct QuoteResponse {
    pub quotes: Vec<QuoteDTO>,
    pub pages: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TagResponse {
    pub tags: Vec<service::data_transfer_objects::TagDTO>,
    pub pages: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorResponse {
    pub authors: Vec<service::data_transfer_objects::AuthorDTO>,
    pub pages: u64,
}

pub async fn get_authors(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_authors_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((authors, pages))) => (
            StatusCode::OK,
            Json(json!(AuthorResponse { authors, pages })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No authors found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn get_quotes(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_quotes_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((quotes, pages))) => {
            (StatusCode::OK, Json(json!(QuoteResponse { quotes, pages })))
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No quotes found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn post_quote(
    state: State<AppState>,
    Json(quote_create_dto): Json<QuoteCreateDTO>,
) -> impl IntoResponse {
    match DataAccess::create_quote(&state.db_conn, quote_create_dto).await {
        Ok(quote_dto) => (StatusCode::CREATED, Json(json!(quote_dto))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn get_tags(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_tags_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((tags, pages))) => (
            StatusCode::OK,
            Json(json!({ "tags": tags, "pages": pages })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No tags found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn get_single_quote(
    state: State<AppState>,
    axum::extract::Path(quote_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match DataAccess::get_quote(&state.db_conn, quote_id).await {
        Ok(Some(quote_dto)) => (StatusCode::OK, Json(json!(quote_dto))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Quote not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn delete_quote(
    state: State<AppState>,
    axum::extract::Path(quote_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match DataAccess::delete_quote(&state.db_conn, quote_id).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn get_tag_and_associated_quotes(
    state: State<AppState>,
    axum::extract::Path(tag_id): axum::extract::Path<i32>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_tag_with_related_quotes(&state.db_conn, tag_id, page, page_size).await {
        Ok(Some((tag, quotes, pages))) => (
            StatusCode::OK,
            Json(json!({ "tag": tag, "quotes": quotes, "pages": pages })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Tag not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn delete_tag(
    state: State<AppState>,
    axum::extract::Path(tag_id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match DataAccess::delete_tag(&state.db_conn, tag_id).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}

pub async fn get_author_and_associated_quotes(
    state: State<AppState>,
    axum::extract::Path(author_id): axum::extract::Path<i32>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_author_with_related_quotes(&state.db_conn, author_id, page, page_size).await {
        Ok(Some((author, quotes, pages))) => (
            StatusCode::OK,
            Json(json!({ "author": author, "quotes": quotes, "pages": pages })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Author not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}