// Cons List
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Using a `Box<T>` to store data on the heap
    {
        let b = Box::new(5);
        println!("b = {}", b); // We can access the data in the box in a similar way 
                               // as we would if this data was on the stack
    } // When it goes out of scope it will be deallocated.

    // Boxes enable recursive types
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("The value of the list is: {:?}", list);
}
