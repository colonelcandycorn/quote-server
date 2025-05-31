use askama::Template;
use serde::Deserialize;
use service::data_access::DataAccess;
use service::data_transfer_objects::QuoteDTO;

use super::AppState;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};

#[derive(Template)]
#[template(path = "./quotes.html")]
struct QuotesTemplate {
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

pub async fn get_root() -> Response {
    Redirect::to("/quotes").into_response()
}
