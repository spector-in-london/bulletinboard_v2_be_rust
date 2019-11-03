use crate::api;
use crate::model;

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

fn get_comments() -> Result<model::Comments> {
    let comments = model::Comments{
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

fn post_comments(comment: model::Comment) -> Result<model::Comment> {
    let response = model::create_comment(comment);

    Ok(response)
}

pub fn post_comments_handler(form: web::Json<FormBody>) -> Result<HttpResponse> {
    let comment = model::Comment {
        id: None,
        title: form.title.to_string(),
        name: form.name.to_string(),
        body: form.body.to_string(),
        avatar: "bunny".to_string(),
    };

    let response = match post_comments(comment) {
        Ok(data) => api::ApiResponse::success(data),
        Err(_err) =>api::ApiResponse::error(),
    };

    Ok(HttpResponse::Ok().json(response))
}
