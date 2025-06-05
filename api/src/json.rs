use crate::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use service::data_access::DataAccess;
use service::data_transfer_objects::{AuthorDTO, QuoteCreateDTO, QuoteDTO, TagCreateDTO, TagDTO};
use utoipa::{IntoParams, OpenApi, ToSchema};

#[derive(Deserialize, Serialize, IntoParams)]
pub struct Params {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QuoteResponse {
    pub quotes: Vec<QuoteDTO>,
    pub pages: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TagResponse {
    pub tags: Vec<TagDTO>,
    pub pages: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthorResponse {
    pub authors: Vec<AuthorDTO>,
    pub pages: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TagAndRelatedQuotesResponse {
    pub tag: TagDTO,
    pub quotes: Vec<QuoteDTO>,
    pub pages: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthorAndAssociatedQuotesResponse {
    pub author: AuthorDTO,
    pub quotes: Vec<QuoteDTO>,
    pub pages: u64,
}

// source: https://github.com/juhaku/utoipa/blob/master/examples/simple-axum/src/main.rs
#[derive(OpenApi)]
#[openapi(paths(
    openapi,
    get_authors,
    get_quotes,
    post_quote,
    get_tags,
    get_single_quote,
    delete_quote,
    get_tag_and_associated_quotes,
    delete_tag,
    get_author_and_associated_quotes,
    patch_quote_with_new_tag
))]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[utoipa::path(
    get,
    path = "/authors",
    params(Params),
    responses(
        (status = 200, description = "List of authors", body = AuthorResponse),
        (status = 404, description = "No authors found"),
        (status = 500, description = "Internal server error")
))]
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

#[utoipa::path(
    get,
    path = "/quotes",
    params(Params),
    responses(
        (status = 200, description = "List of quotes", body = QuoteResponse),
        (status = 404, description = "No quotes found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_quotes(state: State<AppState>, Query(params): Query<Params>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_quotes_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((quotes, pages))) => (StatusCode::OK, Json(json!(QuoteResponse { quotes, pages }))),
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

#[utoipa::path(
    post,
    path = "/quotes",
    request_body = QuoteCreateDTO,
    responses(
        (status = 201, description = "Quote created", body = QuoteDTO),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    get,
    path = "/tags",
    params(Params),
    responses(
        (status = 200, description = "List of tags", body = TagResponse),
        (status = 404, description = "No tags found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tags(state: State<AppState>, Query(params): Query<Params>) -> impl IntoResponse {
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

#[utoipa::path(
    get,
    path = "/quotes/{quote_id}",
    responses(
        (status = 200, description = "Single quote", body = QuoteDTO),
        (status = 404, description = "Quote not found"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    delete,
    path = "/quotes/{quote_id}",
    responses(
        (status = 204, description = "Quote deleted"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    get,
    path = "/tags/{tag_id}",
    params(Params),
    responses(
        (status = 200, description = "Tag and associated quotes", body = TagAndRelatedQuotesResponse),
        (status = 404, description = "Tag not found"),
        (status = 500, description = "Internal server error")
    )
)]
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
            Json(json!(TagAndRelatedQuotesResponse { tag, quotes, pages })),
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

#[utoipa::path(
    delete,
    path = "/tags/{tag_id}",
    responses(
        (status = 204, description = "Tag deleted"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    get,
    path = "/authors/{author_id}",
    params(Params),
    responses(
        (status = 200, description = "Author and associated quotes", body = AuthorAndAssociatedQuotesResponse),
        (status = 404, description = "Author not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_author_and_associated_quotes(
    state: State<AppState>,
    axum::extract::Path(author_id): axum::extract::Path<i32>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_author_with_related_quotes(&state.db_conn, author_id, page, page_size)
        .await
    {
        Ok(Some((author, quotes, pages))) => (
            StatusCode::OK,
            Json(json!(AuthorAndAssociatedQuotesResponse {
                author,
                quotes,
                pages
            })),
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

#[utoipa::path(
    patch,
    path = "/quotes/{quote_id}",
    request_body = TagCreateDTO,
    responses(
        (status = 200, description = "Quote updated with new tag", body = QuoteDTO),
        (status = 404, description = "Quote not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn patch_quote_with_new_tag(
    state: State<AppState>,
    axum::extract::Path(quote_id): axum::extract::Path<i32>,
    Json(tag): Json<TagCreateDTO>,
) -> impl IntoResponse {
    match DataAccess::update_quote_with_new_tag(&state.db_conn, quote_id, tag.tag).await {
        Ok(Some(quote_dto)) => (StatusCode::OK, Json(json!(quote_dto))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Quote not found!"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Internal error: {}", e) })),
        ),
    }
}
