mod api;
mod comments;

use actix_web::{web};

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/api/comments", web::get().to(comments::get_comments_handler))
            .route("/api/comments", web::post().to(comments::post_comments_handler))
    })
    .bind("127.0.0.1:4000")
    .unwrap()
    .run()
    .unwrap();
}
