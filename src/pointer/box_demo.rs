use crate::pointer::box_demo::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn run() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}