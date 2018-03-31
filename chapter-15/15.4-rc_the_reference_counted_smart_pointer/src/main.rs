use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    println!("The value of a is: {:?}", a);
    println!(">>> Count after creating a is: {}.", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("The value of b is: {:?}", b);
    println!(">>> Count after creating b is: {}.", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("The value of c is: {:?}", c);
        println!(">>> Count after creating c is: {}.", Rc::strong_count(&a));
    }
    println!(">>> Count after c goes out of scope is: {}.", Rc::strong_count(&a));
}
