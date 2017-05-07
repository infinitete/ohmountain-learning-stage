extern crate uuid;
extern crate redis;
extern crate iron;
extern crate router;
extern crate hyper;

use uuid::Uuid;
use redis::Commands;
use redis::RedisResult;
use iron::prelude::*;
use iron::status;
use iron::typemap::TypeMap;
use hyper::header::{Headers, Server, ContentLength};

pub struct Validator<'a> {
    pub con: Option<&'a redis::Connection>
}

impl<'a> Validator<'a> {

    pub fn generate(&'a self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn validate(&'a self, key: Option<String>, val: Option<String>) -> bool {

        if key.is_none() || val.is_none() {
            return false;
        }

        let key = key.unwrap();
        let val = val.unwrap();

        let data = self.get(key.as_str());

        if data.is_none() {
            return false;
        }

        match data {
            Some(data) => val.eq(&data),

            None => false
        }
    }

    pub fn get(&'a self, key: &str) -> Option<String> {

        if self.con.is_none() {
            return None;
        }


        let con = self.con.unwrap();

        match con.get(key) {
            Ok(data) => data,
            Err(_) => None
        }

    }

    pub fn set(&'a self, key: &str, val: String, ttl: u16) -> RedisResult<()> {

        let con = self.con.unwrap();
        try!(con.set(key, val));
        try!(redis::cmd("expire").arg(key).arg(ttl).query(con));

        Ok(())
    }

}

pub struct Controller {
    pub ttl: u16,
}

impl Controller {
    pub fn generate(&self) -> Response {

        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let con = client.get_connection().unwrap();

        let validator = Validator { con: Some(&con) };

        let key = validator.generate();
        let val = validator.generate();

        validator.set(key.as_str(), val.clone(), 60);

        let data = format!("{{\"{}\":\"{}\",\"ttl\":{}}}",key, val, self.ttl);
        let mut headers = Headers::new();
        let json: &str = "application/json; charset=utf8";

        headers.set_raw("content-type", vec![json.as_bytes().to_vec()]);
        headers.set(Server("mkd 0.1.0".to_owned()));
        headers.set(ContentLength(data.len() as u64));

        let response = Response {
            status: Some(status::Ok),
            body: Some(Box::new(data)),
            headers: headers,
            extensions: TypeMap::new(),
        };

        response
    }

    pub fn validate(&self, key: Option<String>, val: Option<String>) -> Response {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let con = client.get_connection().unwrap();

        let validator = Validator { con: Some(&con) };

        let validate = validator.validate(key, val);

        let data = format!("{{\"validate\": {}}}", validate);
        let mut headers = Headers::new();
        let json: &str = "application/json; charset=utf8";

        headers.set_raw("content-type", vec![json.as_bytes().to_vec()]);
        headers.set(Server("mkd 0.1.0".to_owned()));
        headers.set(ContentLength(data.len() as u64));

        let response = Response {
            status: Some(status::Ok),
            body: Some(Box::new(data)),
            headers: headers,
            extensions: TypeMap::new(),
        };

        response
    }
}
