use askama::Template;
use serde::Deserialize;
use service::data_access::DataAccess;
use service::data_transfer_objects::QuoteCreateDTO;
use service::data_transfer_objects::QuoteDTO;
use service::data_transfer_objects::AuthorDTO;
use service::data_transfer_objects::TagDTO;

use super::AppState;

use axum::{
    extract::{Query, State, Path, Form},
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

#[derive(Template)]
#[template(path = "./authors.html")]
struct AuthorsTemplate {
    authors: Vec<AuthorDTO>,
    pages: u64
}

#[derive(Template)]
#[template(path = "./tags.html")]
struct TagsTemplate {
    tags: Vec<TagDTO>,
    pages: u64,
}

#[derive(Template)]
#[template(path = "./quote_form.html")]
struct QuoteFormTemplate {
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
        struct ErrorTemplate<'a> {
            status_code: &'a str,
        }
        
        let (status, status_string) = match &self {
            AppError::Render(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
        };
        let tmpl = ErrorTemplate {
            status_code: status_string,
        };
        
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

#[axum::debug_handler]
pub async fn get_authors (
    state: State<AppState>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_authors_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((authors, pages))) => {
            let authors_template = AuthorsTemplate { authors, pages };

            Ok(Html(authors_template.render()?))
        }
        Ok(None) => {
            let authors_template = AuthorsTemplate { authors: Vec::new(), pages: 0 };
            Ok(Html(authors_template.render()?))
        },
        Err(e) => Err(AppError::Database(e)),
    }
}

#[axum::debug_handler]
pub async fn get_tags(
    state: State<AppState>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match DataAccess::get_tags_in_page(&state.db_conn, page, page_size).await {
        Ok(Some((tags, pages))) => {
            let tags_template = TagsTemplate { tags, pages };

            Ok(Html(tags_template.render()?))
        }
        Ok(None) => {
            let tags_template = TagsTemplate { tags: Vec::new(), pages: 0 };
            Ok(Html(tags_template.render()?))
        },
        Err(e) => Err(AppError::Database(e)),
    }
}

#[axum::debug_handler]
pub async fn get_quote_form(
) -> Result<impl IntoResponse, AppError> {
    let quote_form_template = QuoteFormTemplate {};

    Ok(Html(quote_form_template.render()?))
}

#[axum::debug_handler]
pub async fn post_quote_form(
    state: State<AppState>,
    Form(quote_dto): Form<QuoteCreateDTO>
) -> Result<impl IntoResponse, AppError> {
    let _ = DataAccess::create_quote(&state.db_conn, quote_dto).await?;

    // probably should flash a message but idk how to do that right now
    Ok(Redirect::to("/quotes").into_response())
}

pub async fn get_root() -> Response {
    Redirect::to("/quotes").into_response()
}
