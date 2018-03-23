// Testing a private function
fn internal_adder(a: i32, b: i32) -> i32 {
    return a + b;
}

// Integration tests

// The tests directory
pub fn add_two(a: i32) -> i32 {
    return internal_adder(a, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Testing a private function
    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }

    #[test]
    fn adds_two() {
        assert_eq!(add_two(2), 4);
    }
}
