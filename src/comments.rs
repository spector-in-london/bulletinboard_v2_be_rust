use crate::api;
use crate::model;

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    title: String,
    name: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct Comments {
    comments: Vec<model::Comment>,
}

fn get_comments() -> Result<Comments> {
    let comments = Comments{
        comments: model::get_comments(),
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
        title: "title".to_string(),
        name: "name".to_string(),
        body: "body".to_string(),
    };

    Ok(comment)
}

pub fn post_comments_handler(form: web::Json<FormBody>) -> Result<HttpResponse> {
    let data = Comment {
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
