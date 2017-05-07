extern crate iron;
extern crate router;
extern crate micro_token;
extern crate params;

use micro_token::{Controller };
use iron::prelude::*;
use router::Router;
use params::{Params, Value};

fn main() {
    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        let controller = Controller {ttl: 60};
        let response = controller.generate();

        return Ok(response);
    }, "generate");

    router.post("/validate", |request: &mut Request| {

        let controller = Controller {ttl: 60};

        let map = request.get_ref::<Params>().unwrap();

        let key = map.find(&[&"key"]);
        let val = map.find(&[&"val"]);

        let mut _key = Some(String::new());
        let mut _val = Some(String::new());
        let mut response: Response = Response::new();

        if key.is_none() || val.is_none() {
            response = controller.validate(None, None);
        } else {
            match key {
                Some(&Value::String(ref k)) => {
                    _key = Some(k.clone());
                },

                _ => { _key = None  }
            };

            match val {
                Some(&Value::String(ref v)) => {
                    _val = Some(v.clone());
                },

                _ => { _val = None  }
            };

            response = controller.validate(_key, _val);
        }

        Ok(response)

    }, "validate");

    println!("Server runing: 0.0.0.0:3000");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
