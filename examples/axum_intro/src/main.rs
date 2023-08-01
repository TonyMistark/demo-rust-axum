#![allow(unused)] // For beginning only

use axum::extract::Path;
use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::routing::get_service;

// cargo watch -q -c -w src -x run
#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // region:          --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8090));
    println!("->> LISTENING on http://{addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion:       --- Start Server
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:              --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello1/", get(handler_hello1))
        .route("/hello2/:name/", get(handler_hello2))
}

// e.g. `/hello`
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong> World!!!</strong")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g. `/hello1/?name=ice`
async fn handler_hello1(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello 1 <strong> {name} </strong"))
}

// e.g. `/hello2/ice/`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name}", "HANDLER");

    Html(format!("Hello 2 <strong> {name} </strong"))
}
