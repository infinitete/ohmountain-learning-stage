use std::thread;

fn main() {

    let x = 10;

    let handle = thread::spawn(move || {
        println!("Hello World, I am {}", x);
    });

    handle.join().unwrap();
}
