#![allow(unused)]
use crate::log::log_request;

// For beginning only
pub use self::error::{Error, Result};
use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::middleware;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::routing::get_service;
use axum::{Json, Router};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

// cargo watch -q -c -w src -x run
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelContoller.
    let mc = model::ModelContoller::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
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

async fn main_repsonse_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("--->> {:<12} - main_reponse_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    // -- Get the eventual repsonse error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|secv| secv.client_status_and_error());

    // -- If client error, build the new repsonse.
    let error_reponse = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!("  ->> client_error_body: {client_error_body}");

            // Build the new repsonse from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // -- Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_reponse.unwrap_or(res)
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
