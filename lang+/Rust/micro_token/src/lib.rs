extern crate uuid;
extern crate redis;
extern crate iron;
extern crate router;
extern crate hyper;
extern crate regex;

use uuid::Uuid;
use redis::Commands;
use redis::RedisResult;
use iron::prelude::*;
use iron::status;
use iron::typemap::TypeMap;
use hyper::header::{Headers, Server, ContentLength};
use regex::Regex;
use std::f64;
use std::str::FromStr;


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
pub struct ParameterBuilder {
}

impl ParameterBuilder {
    pub fn new(body: String) -> ParameterHandler {
        ParameterHandler {
            body: body.clone(),
            parameters: None
        }
    }
}

pub struct ParameterHandler {
    pub body: String,
    parameters: Option<Vec<Parameter>>,
}

impl ParameterHandler {

    pub fn get_paramaters(&mut self) -> Vec<Parameter> {

        if self.parameters.is_some() {
            return self.parameters.clone().unwrap();
        }

        let reg = Regex::new("(?P<key>^[a-zA-Z]\\w+)=(?P<val>.+)").unwrap();

        let pairs: Vec<&str> = self.body.split('&').collect();

        let mut ret:Vec<Parameter> = Vec::new();

        for kv in pairs {
            for cp in reg.captures_iter(kv) {
                ret.push(Parameter{
                    key: cp[1].trim().to_string().clone(),
                    val: cp[2].trim().to_string().clone()
                });
            }
        }

        self.parameters = Some(ret.clone());

        ret
    }

    pub fn get(&mut self, key: String) -> Option<String> {

        if self.parameters.is_none() {
            self.get_paramaters();
        }

        let parameters_vec = self.parameters.clone();

        if parameters_vec.is_none() {
            return None
        }

        for kv in parameters_vec.unwrap() {
            if kv.key.eq(&key) {
                return Some(kv.val.clone());
            }
        }

        None
    }
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Parameter {
    pub key: String,
    pub val: String
}

impl Parameter {
    pub fn get_numeric(&self) -> Option<f64> {

        let f64_val = f64::from_str(self.val.clone().as_str());

        match f64_val {
            Ok(data) => Some(data),
            Err(_) => None
        }
    }
}
