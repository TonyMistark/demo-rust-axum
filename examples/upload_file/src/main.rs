use axum::{
    extract::Multipart,
    routing::post,
    Router,
};

async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes, {:?}", name, data.len(), data);
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    // async {
    // };
}

