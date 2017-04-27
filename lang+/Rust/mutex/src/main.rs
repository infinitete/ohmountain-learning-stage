use std::sync::Mutex;
use std::mem;
use std::env::consts::{OS, ARCH, FAMILY};

fn main() {
    let data = Mutex::new(0);

    println!("{}, {}, {}, {}, {}", data.into_inner().unwrap(), mem::size_of_val("ss"), OS, ARCH, FAMILY);
}
