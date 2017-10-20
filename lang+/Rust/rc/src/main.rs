use std::rc::Rc;

fn main() {

    let rc = Rc::new(1);
    println!("{}", Rc::strong_count(&rc));

    let a = rc.clone();
    println!("{}", Rc::strong_count(&rc));

    let b = rc.clone();
    println!("{}", Rc::strong_count(&rc));

    println!("{:?}", b);
}
