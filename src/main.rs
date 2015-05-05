extern crate iron;

use iron::request::Request;
use iron::response::Response;
use iron::Iron;
use iron::status;
use iron::static::Static;

fn main() {
    Iron::new(Static::new("../public")).http("localhost::1337").unwrap();
}
