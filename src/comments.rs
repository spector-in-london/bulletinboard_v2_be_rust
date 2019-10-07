use crate::api;

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

pub fn get_comments_handler() -> Result<HttpResponse> {
    let data = Comments{
        comments: vec![Comment {
            id: 1,
            title: "Hello World".to_string(),
            name: "Robert".to_string(),
            body: "This is my first Rust server".to_string(),
        }],
    };

    let response = api::ApiResponse::success(data);
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Serialize, Deserialize)]
pub struct FormBody {
    title: String,
    name: String,
    body: String,
}

pub fn post_comments_handler(form: web::Json<FormBody>) -> Result<HttpResponse> {
    let data = Comment {
        id: 1,
        title: form.title.to_string(),
        name: form.name.to_string(),
        body: form.body.to_string(),
    };

    let response = api::ApiResponse::success(data);
    Ok(HttpResponse::Ok().json(response))
}
