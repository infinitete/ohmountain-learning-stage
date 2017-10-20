use std::cell::RefCell;

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;

    println!("b is {}", b);
}

fn demo(cell: RefCell<i32>) {
    a_fn_that_immutably_borrows(&cell.borrow());
    a_fn_that_mutably_borrows(&mut cell.borrow_mut());
    a_fn_that_immutably_borrows(&cell.borrow());
}

fn main() {

    // 非mut
    let i = RefCell::new(5);

    demo(i);
}
