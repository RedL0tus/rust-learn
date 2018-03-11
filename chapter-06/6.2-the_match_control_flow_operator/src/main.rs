fn main() {
    println!("This coin is a {:?}, it worths {} cents.", Coin::Penny, value_in_cents(Coin::Penny));
    println!("This coin is a {:?}, it worths {} cents.", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("This coin is a {:?}, it worths {} cents.", Coin::Dime, value_in_cents(Coin::Dime));
    println!("This coin is a {:?}, it worths {} cents.", Coin::Quarter(UsState::California), value_in_cents(Coin::Quarter(UsState::California)));
    
    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // The _ Placeholder
    let some_u8_value = 1;
    match some_u8_value {
        1 => println!("One!"),
        3 => println!("Three!"),
        5 => println!("Five!"),
        7 => println!("Seven!"),
        _ => (), // Matches any value that aren't specified before it
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Rust will throw an error if this line is removed (None not covered).
        Some(i) => Some(i + 1),
    }
}
