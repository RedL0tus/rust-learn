#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    // Type Aliases Create Type Synonyms
    {
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
        type Thunk = Box<Fn() + Send + 'static>;

        // Type aliases can reduce the repetition
        let f: Thunk = Box::new(|| println!("hi"));
        fn takes_long_type(f: Thunk) {
            // --snip--
        }
        fn returns_long_type() -> Thunk {
            // --snip--
            return Box::new(|| println!("Hi"));
        }
    }
    // The `!` Never Type that Never Returns
    {
        fn bar() -> ! {
            print!("Forever ");
            loop {
                print!("and ever ");
            }
        }
    }
    // Dynamically Sized Types & `Sized`
    {
        // The code below won't work
        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";
        
        fn generic_orig<T>(t: T) {
            // --snip--
        }
        // Is actually treated as this:
        fn generic_actually<T: Sized>(t: T) {
            // --snip--
        }
        // This restriction can be relaxed by:
        fn generic_relaxed<T: ?Sized>(t: &T) {
            // --snip--
        }
    }
}
