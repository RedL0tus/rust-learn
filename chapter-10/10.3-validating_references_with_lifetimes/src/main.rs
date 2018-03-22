use std::fmt::Display;

fn main() {
    borrow_checker();

    // Annotating lifetime
    let x = String::from("ABCD");
    let y = String::from("EFGJIJ");
    let longest = longest_with_an_announcement(&x, &y, String::from("Bazinga!"));
    println!("{}", longest);

    // Lifetime annotations in struct definitions
    lifetime_in_struct_definitions();

    // Lifetime Elision
    println!("The first word is: {}.", first_word(&String::from("Salted Fish")));

    // Static Lifetime
    //
    // `'static` lifetime is the entire duration of the program.
    // All String literals have `'static` lifetime.
    // Can be annotated as below.
    let s: &'static str = "I have a static lifetime!";
    println!("{}", s);
}

fn borrow_checker() {
    // Lifetime of each variable
    // let r;         // -------+-- 'a
    //                //        |
    // {              //        |
    //     let x = 5; // -+-----+-- 'b
    //     r = &x;    //  |     |
    // }              // -+     |
    //                //        |
    // println!("r: {}", r); // | // Will result in an error, lifetime of r is longer than x.
    //                //        |
    //                // -------+
}

// Lifefime annotation with generic type parameters and trait bounds
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

// Lifetime annotations in Struct definitions
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> { // Lifetime annotations in method definition
    fn level(&self) -> i32 {
        return 3;
    }
}

fn lifetime_in_struct_definitions() {
    let novel = String::from("Call me Salted Fish. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}", i);
    println!("{}", i.level());
}

// Lifetime Elision
//
// Sometimes a function can be defined without the annotation of lifetime
// As long as it follow several rules called Lifetime Elision.
// the first_word function is an example of it.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
