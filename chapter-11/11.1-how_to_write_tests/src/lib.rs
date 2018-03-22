// Checking Results with the `assert!` Macro
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        return self.length > other.length && self.width > other.width;
    }
}

// Testing Equality with the `assert_eq!` and `assert_ne!` Macros
pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

// Adding Custom Failure Message
pub fn greeting(name: &str) -> String {
    return format!("Hello {}!", name);
}

// Checking for Panics with `should_panic`
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        return Guess {
            value,
        }
    }

    pub fn print_value(&self) {
        println!("{}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Make this test fail using panic!
    #[test]
    fn die_die_die() {
        panic!("Make this test fail again!");
    }

    // Checking Results with the `assert!` Macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // Testing Equality with the `assert_eq!` and `assert_ne!` Macros
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    // assert_ne! is equal to !=

    // Adding Custom Failure Message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    // Checking for Panics with `should_panic`
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
