use std::fmt::Display;

struct CustomSmartPointer<T: Display> {
    data: T,
}

impl<T> Drop for CustomSmartPointer<T> 
    where T: Display {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!(">>> Begin of main()");
    let c = CustomSmartPointer { data: String::from("My stuff") };
    let d = CustomSmartPointer { data: String::from("Other stuff") };
    println!(">>> CustomSmartPointers c&d are created.");
    println!("c has a value of `{}`.", c.data);
    println!("d has a value of `{}`.", d.data);
    // Dropping a value with `std::mem::drop`
    // c.drop(); // Explicit destructor calls not allowed
    drop(c);
    println!(">>> CustomSmartPointer c dropped before the end of main.");
    println!(">>> End of main()");
}
