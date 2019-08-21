use std::env;

#[derive(Debug)]
pub struct Args {
    pub query: String,
    pub filename: String,
}




pub fn get_args() -> Option<Args> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        return None
    }

    Some(Args {
        query: args[0].clone(),
        filename: args[1].clone()
    })
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_get_args() {
        println!("{:?}", super::get_args());
    }

}
