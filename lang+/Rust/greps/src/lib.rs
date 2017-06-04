use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file:  String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough params");
        }

        let query = args[1].clone();
        let file  = args[2].clone();

        Ok(Config {
            query: query,
            file:  file
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<Error>>{
    // let mut f = File::open(config.file).unwrap_or_else(|err| {
    //     println!("Open file error: {}", err);
    //     process::exit(1);
    // });

    let mut  f = File::open(config.file)?;

    let mut content = String::new();

    f.read_to_string(&mut content)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}
