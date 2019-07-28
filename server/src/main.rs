use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    id: u32,
    title: String,
    name: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct Comments {
    comments: Vec<Comment>
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
    message: String,
}

fn getCommentsHandler() -> Result<HttpResponse> {
    let data = Comments{
        comments: vec![Comment {
            id: 1,
            title: "Hello World".to_string(),
            name: "Robert".to_string(),
            body: "This is my first Rust server".to_string(),
        }],
    };

    let response = ApiResponse::<Comments> {
        status: "success".to_string(),
        data: data,
        message: "".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/api/comments", web::get().to(getCommentsHandler))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
