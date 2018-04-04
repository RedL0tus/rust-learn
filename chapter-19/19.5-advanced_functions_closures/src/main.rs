fn main() {
    // Function Pointers
    {
        fn add_one(x: i32) -> i32 {
            return x + 1;
        }
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            return f(arg) + f(arg);
        }
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}.", answer);
    }
    // Returning Closures
    {
        fn returns_closure() -> Box<Fn(i32) -> i32> {
            return Box::new(|x| x + 1); // Needs to be wrapped by `Box<T>`
        }
        println!("{}", returns_closure()(5));
    }
}
