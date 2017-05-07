extern crate iron;
extern crate router;
extern crate micro_token;

use micro_token::{Controller, ParameterBuilder, ParameterHandler};
use iron::prelude::*;
use router::Router;
use std::io::Read;
use iron::status;

fn main() {
    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        let controller = Controller {ttl: 60};
        let response = controller.generate();

        return Ok(response);
    }, "generate");

    router.post("/validate", |request: &mut Request| {

        let controller = Controller {ttl: 60};
        let mut body: String = String::new();

        request.body.read_to_string(&mut body);

        let mut parameter_handler: ParameterHandler = ParameterBuilder::new(body);

        let key_name = "key".to_string();
        let val_name = "val".to_string();

        let key = parameter_handler.get(key_name);
        let val = parameter_handler.get(val_name);

        let response = controller.validate(key, val);

        Ok(response)

    }, "validate");

    println!("Server runing: 0.0.0.0:3000");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
