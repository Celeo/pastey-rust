use axum::prelude::*;
use log::{debug, info};
use std::{net::SocketAddr, path::Path};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

mod data;
use data::setup_db;

mod routes;

#[tokio::main]
async fn main() {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Could not load .env file");
    }
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "pastey=info");
    }
    pretty_env_logger::init();

    debug!("Setting up db");
    setup_db().await.expect("Could not setup DB");

    debug!("Setting up service");
    let logging_middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .into_inner();
    let app = route("/", get(routes::index))
        .route("/view/:uuid", get(routes::view_paste))
        .route("/new", get(routes::new_paste_page))
        .route("/new/save", post(routes::new_paste_page_save))
        .layer(logging_middleware);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Starting");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
