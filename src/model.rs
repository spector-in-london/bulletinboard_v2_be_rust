extern crate postgres;

use postgres::{Connection, TlsMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<i32>,
    pub title: String,
    pub name: String,
    pub avatar: String,
    pub body: String,
}

pub fn get_comments() -> Vec<Comment> {
    let sql = "SELECT * FROM posts";

    let conn = match Connection::connect("postgres://robertschaap@localhost/bulletinboard", TlsMode::None) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };

    let mut comments: Vec<Comment> = Vec::new();

    for row in &conn.query(sql, &[]).unwrap() {
        comments.push(Comment {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            avatar: row.get("avatar"),
            name: row.get("name"),
        });
    }

    return comments;
}

pub fn create_comment(comment: Comment) -> Comment {
    let sql = "INSERT INTO posts (name, title, body, avatar) VALUES ()";
    println!("{}", sql);

    return comment;
}
