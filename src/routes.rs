use crate::data;
use axum::{
    body::{box_body, BoxBody},
    extract::Path,
    prelude::*,
    response::IntoResponse,
};
use http::{Response, StatusCode};
use log::error;
use once_cell::sync::Lazy;
use tera::{Context, Tera};

static TERA_INSTANCE: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("base", include_str!("templates/base.html")),
        ("index", include_str!("templates/index.html")),
        ("view", include_str!("templates/view.html")),
        ("new_paste", include_str!("templates/new_paste.html")),
    ])
    .unwrap();
    tera
});

pub async fn index() -> impl IntoResponse {
    response::Html(
        TERA_INSTANCE
            .render("index", &Context::new())
            .expect("Could not load index template"),
    )
}

pub async fn view_paste(Path(uuid): Path<String>) -> impl IntoResponse {
    let info = match data::get_paste(&uuid).await {
        Ok(Some(info)) => info,
        _ => return RedirectResponse::new("/").into_response(),
    };

    let mut context = Context::new();
    context.insert("info", &info);
    let template = match TERA_INSTANCE.render("index", &context) {
        Ok(t) => t,
        Err(e) => {
            error!("Error loading view paste page: {}", e);
            return RedirectResponse::new("/").into_response();
        }
    };

    response::Html(template).into_response()
}

pub async fn new_paste_page() -> &'static str {
    "new_paste_page()"
}

pub async fn new_paste_page_save() -> &'static str {
    "new_paste_page_save()"
}

pub fn map_404(response: Response<BoxBody>) -> Response<BoxBody> {
    if response.status() == StatusCode::NOT_FOUND
        || response.status() == StatusCode::METHOD_NOT_ALLOWED
    {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(box_body(Body::from("404")))
            .unwrap();
    }
    response
}

// ======================================================

// #[derive(Template)]
// #[template(path = "index.html")]
// struct IndexTemplate {}

// #[derive(Template)]
// #[template(path = "view.html")]
// struct ViewTemplate {
//     info: PasteInfo,
// }

// #[derive(Template)]
// #[template(path = "new_paste.html")]
// struct NewPasteTemplate {}

// struct HtmlTemplate<T>(T);

// impl<T> IntoResponse for HtmlTemplate<T>
// where
//     T: Template,
// {
//     fn into_response(self) -> Response<axum::body::Body> {
//         match self.0.render() {
//             Ok(html) => response::Html(html).into_response(),
//             Err(e) => Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(format!("Error processing template: {}", e).into())
//                 .unwrap(),
//         }
//     }
// }

struct RedirectResponse {
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

impl RedirectResponse {
    fn new(to: &str) -> Self {
        Self { to: to.to_owned() }
    }
}
