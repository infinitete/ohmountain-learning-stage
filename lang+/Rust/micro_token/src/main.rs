extern crate iron;
extern crate router;
extern crate hyper;
extern crate redis;
extern crate uuid;
extern crate micro_token;

use micro_token::{Controller };
use iron::prelude::*;
use router::Router;


fn main() {
    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        let controller = Controller {ttl: 60};
        let response = controller.generate();

        return Ok(response);
    }, "generate");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
