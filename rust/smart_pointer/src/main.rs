use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("{b}");
    let a = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));
   
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

