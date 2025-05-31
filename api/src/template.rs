use askama::Template;
use serde::Deserialize;
use service::data_access::DataAccess;
use service::data_transfer_objects::QuoteDTO;

use super::AppState;

use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};

#[derive(Template)]
#[template(path = "./quotes.html")]
struct QuotesTemplate {
    quotes: Vec<QuoteDTO>,
    pages: u64,
}

#[derive(Template)]
#[template(path = "./tag.html")]
struct TagTemplate {
    tag: service::data_transfer_objects::TagDTO,
    quotes: Vec<QuoteDTO>,
    pages: u64,
}

#[derive(Template)]
#[template(path = "./author.html")]
struct AuthorTemplate {
    author: service::data_transfer_objects::AuthorDTO,
    quotes: Vec<QuoteDTO>,
    pages: u64,
}

#[derive(Deserialize)]
pub struct Params {
    page: Option<u64>,
    page_size: Option<u64>,
}

/*
source: https://askama.readthedocs.io/en/stable/frameworks.html
*/
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
    /// Had trouble with database
    Database(#[from] sea_orm::DbErr),
    /// Not Found
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Debug, Template)]
        #[template(path = "./error.html")]
        struct Tmpl {}

        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
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
pub async fn get_quotes(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_quotes_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((quotes, pages))) => {
            let quotes_template = QuotesTemplate { quotes, pages };

            Ok(Html(quotes_template.render()?))
        }
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::Database(e)),
    }
}

#[axum::debug_handler]
pub async fn get_tag_and_associated_quotes(
    state: State<AppState>,
    Path(tag_id): Path<i32>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_tag_with_related_quotes(&state.db_conn, tag_id, page, page_size).await {
        Ok(Some((tag, quotes, pages))) => {
            let quotes_template = TagTemplate { tag, quotes, pages };

            Ok(Html(quotes_template.render()?))
        }
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::Database(e)),
    }
}

#[axum::debug_handler]
pub async fn get_author_and_associated_quotes(
    state: State<AppState>,
    Path(author_id): Path<i32>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_author_with_related_quotes(&state.db_conn, author_id, page, page_size).await {
        Ok(Some((author, quotes, pages))) => {
            let quotes_template = AuthorTemplate { author: author.into(), quotes, pages };

            Ok(Html(quotes_template.render()?))
        }
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::Database(e)),
    }
}

pub async fn get_root() -> Response {
    Redirect::to("/quotes").into_response()
}
