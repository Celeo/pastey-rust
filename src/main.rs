use axum::prelude::*;
use std::net::SocketAddr;

mod data;

#[tokio::main]
async fn main() {
    let app = route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// TODO ...
