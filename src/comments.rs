use crate::api;
use crate::model;

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryParams {
    offset: Option<String>,
    sort: Option<String>,
}

fn get_comments(params: QueryParams) -> Result<model::Comments> {
    let comments = model::Comments{
        comments: model::get_comments(params.offset, params.sort),
    };

    Ok(comments)
}

pub fn get_comments_handler(info: web::Query<QueryParams>) -> Result<HttpResponse> {
    let response = match get_comments(info.into_inner()) {
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
