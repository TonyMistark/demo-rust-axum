use axum;
use axum::routing::get;
use tokio;

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", get(index_handler));

    axum::Server::bind(&"127.0.0.1:8989".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}


async fn index_handler() -> String {
    String::from("Hello World")
}
