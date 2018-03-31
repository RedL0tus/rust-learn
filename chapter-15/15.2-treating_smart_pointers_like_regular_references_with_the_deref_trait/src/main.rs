use std::ops::Deref;

// Defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

// Treating a type like a reference by implementing the `Deref` trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return &self.0;
    }
}

// Implicit deref coercions with functions and methods
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // Following the pointer to the value with `*`
    {
        let x = 5;
        let y = &x;

        println!("Is x equals to 5? {:?}.", x == 5);
        // Must use * here to dereference y
        // println!("Is y equals to 5? {}.", y == 5); // Will not compile
        println!("Is *y equals to 5? {:?}.", *y == 5);
    }
    // Using `Box<T>` like a reference
    {
        let x = 5;
        let y = Box::new(x);
        println!("Is x equals to 5? {:?}.", x == 5);
        // Must use * here to dereference y
        // println!("Is y equals to 5? {}.", y == 5); // Will not compile
        println!("Is *y equals to 5? {:?}.", *y == 5); // Using the dereference operator on a `Box<T>`
    }
    // Defining our own smart pointer
    {
        let x = 5;
        let y = MyBox::new(x);

        println!("Is x equals to 5? {:?}.", x == 5);
        // Must use * here to dereference y
        // println!("Is y equals to 5? {}.", y == 5); // Will not compile
        println!("Is *y equals to 5? {:?}.", *y == 5);
    }
    // Implicit deref coercions with functions and methods
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m); // It will work because of deref coercion
        // Rust does deref coercion when it finds types and trait implementations in three cases:
        // - From `&T` to `&U` when `T: Deref<Target=U>`
        // - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
        // - From `&mut T` to `&U` when `T: Deref<Target=U>`
    }
}
