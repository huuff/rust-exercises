
// fn hello_world_handler

use axum::{response::IntoResponse, Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> impl IntoResponse {
    "Hello world"
}
