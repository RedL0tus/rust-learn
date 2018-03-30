extern crate add_one;
extern crate add_two;
extern crate add_rand;

fn main() {
    let num = 10;
    println!("Hello, world!");
    println!("{} plus one is {}!", num, add_one::add_one(num));
    println!("{} plus two is {}!", num, add_two::add_two(num));
    println!("{} plus a random number is {}!", num, add_rand::add_rand(num));
}
