extern crate iron;

use iron::request::Request;
use iron::response::Response;
use iron::Iron;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hi, Pete")))
    }).http("localhost:1337").unwrap();
}
