use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;

fn log<T: Any + Debug>(value: &T) {

    let any_value = value as &Any;

    match any_value.downcast_ref::<String>() {
        Some(as_string) => {
            println!("String ({}): {}", as_string.len(), as_string);
        },

        None => {
            println!("{:?}", value);
        }
    }

    println!("TypeId: {:?}", TypeId::of::<T>());

}

fn do_work<T: Any + Debug>(value: &T) {
    log(value);
}

fn main() {
    let my_string = "你好，世界".to_string();
    do_work(&my_string);

    let my_i32 = 143i32;
    do_work(&my_i32);
}
