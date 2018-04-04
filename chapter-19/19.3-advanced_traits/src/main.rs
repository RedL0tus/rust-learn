#![allow(dead_code)]
fn main() {
    // Associated Type Specify Placeholder Types in Trait Definitions
    {
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        // Associated Types Versus Generics
        {
            struct Counter {
                count: u32,
            }
            impl Counter {
                fn new() -> Counter {
                    return Counter { count: 0 };
                }
            }
            impl Iterator for Counter {
                type Item = u32;
                fn next(&mut self) -> Option<Self::Item> {
                    self.count += 1;
                    if self.count < 6 {
                        return Some(self.count);
                    } else {
                        return None;
                    }
                }
            }
        }
    }
    // Default Generic Type Parameters and Operator Overloading
    {
        use std::ops::Add;
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        println!("{}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 } == Point { x: 3, y: 3 });
    }
    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
    {
        trait Pilot {
            fn fly(&self);
        }
        trait Wizard {
            fn fly(&self);
        }
        struct Human;
        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }
        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }
        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);
    }
    // Using Supertraits to Require One Trait's Functionality Within Another Trait
    {
        use std::fmt;
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        struct Point {
            x: i32,
            y: i32,
        }
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "({}, {})", self.x, self.y);
            }
        }
        impl OutlinePrint for Point {} // It requires `Display` implemented
        let sample = Point { x: 1, y: 1 };
        sample.outline_print();
    }
    // The Newtype Pattern to Implement External Traits on External Types
    {
        use std::fmt;
        struct Wrapper(Vec<String>);
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
