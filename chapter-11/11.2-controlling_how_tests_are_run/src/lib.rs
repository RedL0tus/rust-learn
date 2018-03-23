#[cfg(test)]
mod tests {
    // Tests can be run in parallel or consecutively
    // with `cargo test -- --test-threads=X`

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // `println!` can be used in test cases with
    // `cargo test -- --nocapture`
    #[test]
    fn println_can_be_used() {
        println!("Println! can also be used in test cases.");
        assert_eq!(2 + 2, 4);
    }

    // Can run only one test at a time with
    // `cargo test test_name`
    
    // The next test will be ignored
    // unless using `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn this_will_be_ignored() {
        assert_eq!(2 + 2, 4);
    }
}
