use axum::{Router, routing::get, extract::Query, response::IntoResponse};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct InputQuery {
    name: Option<String>,
}

async fn hello_world(Query(input_query): Query<InputQuery>) -> impl IntoResponse {
    let name = input_query.name.as_deref().unwrap_or("World");

    format!("Hello, {name}!")
}
