use axum::{
    extract::{Multipart, FromRequest},
    routing::{post, get},
    Router,
    Json,
};
use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use std::convert::AsMut;

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let data_len = data.len();
        if data_len > 100 {
            println!("Length of `{}` is {} bytes", name, data.len());
        } else {
            println!("Length of `{}` is {} bytes, {:?}", name, data.len(), data);
        }
    }
}

struct ImactData {
    old_docx: Vec<u8>,
    new_docx: Vec<u8>,
    lqr: Value,
    robot: Value,
    styles: Value,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ImactData
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request(req: &'a mut Multipart, state: &S) -> Result<Self, Self::Rejection> {
        let mut impact_data: ImactData;
        while  let Some(field) = req.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();
            let mut data = field.bytes().await.unwrap();
            if name == "old_docx" {
                impact_data.old_docx = AsMut::as_mut(&data);
            } else if name == "new_docx" {
                let new_docx: &mut [u8] = AsMut::as_mut(&data);
                impact_data.new_docx = &new_docx;
            } else if name == "lqr" {
                let lqr: &mut [u8] = AsMut::as_mut(&data);
                impact_data.lqr = &lqr;
            }
        }
        Ok(impact_data)
    }
}



async fn orientate(mut multipart: Multipart) {
    while  let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes, {:?}", name, data.len(), data);
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoResp {
    pub title: String,
    pub code: u16,
}

impl CreateTodoResp {
    pub fn new(title: String, code: u16) -> Self {
        CreateTodoResp{title: title, code: code}
    }
}

async fn create_todo_handler(Json(todo): Json<CreateTodo>) -> Json<CreateTodoResp> {
    Json(CreateTodoResp::new(todo.title, StatusCode::CREATED.as_u16()))
}

async fn index_handler() -> String {
    String::from("Hello World")
}

#[derive(Debug)]
enum  HttpError {
    BadRequest,
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "500"),
            HttpError::BadRequest => (StatusCode::BAD_REQUEST, "BadRequest"),
        };
        (code, msg).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload))
        .route("/", get(index_handler))
        .route("/todo", post(create_todo_handler));
    println!("start server: 127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}

