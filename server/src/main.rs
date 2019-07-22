extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn get_comments_handler(_: &mut Request) -> IronResult<Response> {
    let response = "response";
    Ok(Response::with((status::Ok, response)))
}

fn main() {
    let mut router = Router::new();
    router.get("/api/comments", get_comments_handler, "index");

    let address = "localhost:3001";
    let _server = Iron::new(router).http(address).unwrap();

    println!("Running on http://{}", address);
}
