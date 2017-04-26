use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

// 10个线程做0 + ... + 9


fn main() {

    let data = Arc::new(Mutex::new(0));

    // tx 是 sender
    // rx 是 receiver
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {

        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {

            let mut data = data.lock().unwrap();
            *data += i;

            tx.send(|i|{
                i
                //println!("{}", i)
            }).unwrap();
        });
    }

    for i in 0..10 {
        rx.recv().unwrap()(i);
    }

    let m = Arc::try_unwrap(data).unwrap();


    let data = match m.into_inner() {
        Ok(data) => data,
        Err(_) => 0,
    };

    println!("{}", data);

}
