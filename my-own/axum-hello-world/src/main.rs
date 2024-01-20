#[macro_use]
extern crate rust_i18n;
extern crate accept_language;

use axum::{Router, routing::get, extract::Query, response::IntoResponse, http::{HeaderMap, header}};
use serde::Deserialize;
use rust_i18n::t;

i18n!("locales", fallback = "en");

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

async fn hello_world(Query(query): Query<InputQuery>, headers: HeaderMap) -> impl IntoResponse {
    let request_language = headers.get(header::ACCEPT_LANGUAGE)
        // TODO: Error handling
        .map(|it| it.to_str().unwrap());
    let language = request_language
        .map(|request_language| accept_language::intersection(request_language, &rust_i18n::available_locales!()[..]))
        .and_then(|common_languages| common_languages.into_iter().next())
        // TODO: Avoid to_string?
        .unwrap_or(rust_i18n::locale().to_string())
        ;

    // TODO: Avoid to_string?
    let name = query.name.unwrap_or(t!("world", locale = &language).to_string());

    t!("hello_name", locale = &language, name = name)
}
