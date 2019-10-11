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
    comments: Vec<Comment>,
}

fn get_comments() -> Result<Comments> {
    let comments = Comments{
        comments: vec![
            Comment { id: 1, title: "title1".to_string(), name: "name1".to_string(), body: "body1".to_string() },
            Comment { id: 2, title: "title2".to_string(), name: "name2".to_string(), body: "body2".to_string() },
            Comment { id: 3, title: "title3".to_string(), name: "name3".to_string(), body: "body3".to_string() },
            Comment { id: 4, title: "title4".to_string(), name: "name4".to_string(), body: "body4".to_string() },
        ],
    };

    Ok(comments)
}

pub fn get_comments_handler() -> Result<HttpResponse> {
    let response = match get_comments() {
        Ok(data) => api::ApiResponse::success(data),
        Err(_err) => api::ApiResponse::error(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Serialize, Deserialize)]
pub struct FormBody {
    title: String,
    name: String,
    body: String,
}

fn post_comments() -> Result<Comment> {
    let comment = Comment {
        id: 1,
        title: "title".to_string(),
        name: "name".to_string(),
        body: "body".to_string(),
    };

    Ok(comment)
}

pub fn post_comments_handler(form: web::Json<FormBody>) -> Result<HttpResponse> {
    let data = Comment {
        id: 1,
        title: form.title.to_string(),
        name: form.name.to_string(),
        body: form.body.to_string(),
    };

    // TODO: remove drop of data and replace with model response
    let response = match post_comments() {
        Ok(_data) => api::ApiResponse::success(data),
        Err(_err) =>api::ApiResponse::error(),
    };

    Ok(HttpResponse::Ok().json(response))
}
