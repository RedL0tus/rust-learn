struct User { // Define a struct
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32); 
// Though the two types above have the same type of values
// They are still no the same.

struct Unitlike(); // A unit-like struct

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1's email is: {}.", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("User1's email now is: {}.", user1.email);
    let user2 = build_user(String::from("saltedfish@example.com"), String::from("SaltedFish"));
    println!("User2's email is: {}.", user2.email);
    println!("User2's username is: {}.", user2.username);
    // Creating instances from other instances with struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername456"),
        // active: user1.active;
        // sign_in_count: user1.sign_in_count,
        ..user1 // Can be shortened as this
    };
    println!("User3's state is: {}.", user3.active);
    println!("User3's sign in count is: {}.", user3.sign_in_count);

    // Tuple structs without named fields to create different types
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("The value of black is: {}, {}, {}.", black.0, black.1, black.2);
    println!("The value of origin is: {}, {}, {}.", origin.0, origin.1, origin.2);
    let _unit_like = Unitlike(); // A unit like struct
}

fn build_user(email: String, username: String) -> User {
    return User {
        // email: email, // Can be shortened as below
        email,
        // username: username,
        username,
        active: true,
        sign_in_count: 1,
    };
}
