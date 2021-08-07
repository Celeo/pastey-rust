use axum::prelude::*;
use log::{debug, info};
use std::{net::SocketAddr, path::Path};

mod data;
use data::setup_db;

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
    let app = route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Starting");
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
