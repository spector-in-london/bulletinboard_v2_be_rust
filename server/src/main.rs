extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn getCommentsHandler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "response")))
}

fn main() {
    let mut router = Router::new();
    router.get("/api/comments", getCommentsHandler, "index");

    let address = "localhost:3001";
    let _server = Iron::new(router).http(address).unwrap();

    println!("Running on http://{}", address);
}
