#![allow(unused)] // For beginning only
pub use self::error::{Error, Result};
use axum::extract::{Query, Path};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::routing::get_service;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::middleware;
use tower_cookies::CookieManagerLayer;

mod error;
mod web;
mod model;

// cargo watch -q -c -w src -x run
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelContoller.
    let mc = model::ModelContoller::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes())
        .layer(middleware::map_response(main_repsonse_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:          --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8090));
    println!("->> LISTENING on http://{addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion:       --- Start Server

    Ok(())
}

async fn main_repsonse_mapper(res: Response) -> Response {
    println!("--->> {:<12} - main_reponse_mapper", "RES_MAPPER");

    println!("after");

    res
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
