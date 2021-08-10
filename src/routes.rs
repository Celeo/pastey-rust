use askama::Template;
use axum::{extract::Path, prelude::*, response::IntoResponse};
use http::{Response, StatusCode};

use crate::data::{self, PasteInfo};

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

pub async fn view_paste(Path(uuid): Path<String>) -> impl IntoResponse {
    match data::get_paste(&uuid).await {
        Ok(Some(info)) => HtmlTemplate(ViewTemplate { info }).into_response(),
        _ => RedirectResponse {
            to: String::from("/"),
        }
        .into_response(),
    }
}

pub async fn new_paste_page() -> impl IntoResponse {
    HtmlTemplate(NewPasteTemplate {})
}

pub async fn new_paste_page_save() -> impl IntoResponse {
    // TODO
    "new_paste_page_save()"
}

// ======================================================

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "view.html")]
struct ViewTemplate {
    info: PasteInfo,
}

#[derive(Template)]
#[template(path = "new_paste.html")]
struct NewPasteTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response<axum::body::Body> {
        match self.0.render() {
            Ok(html) => response::Html(html).into_response(),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Error processing template: {}", e).into())
                .unwrap(),
        }
    }
}

pub struct RedirectResponse {
    to: String,
}

impl IntoResponse for RedirectResponse {
    fn into_response(self) -> http::Response<axum::body::Body> {
        let builder = http::Response::builder()
            .header("Location", &self.to)
            .status(StatusCode::FOUND);

        builder.body(axum::body::Body::empty()).unwrap()
    }
}
