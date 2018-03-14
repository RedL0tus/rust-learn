mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn main() {
    outermost::middle_function();
    // outermost::middle_secret_function();
    // outermost::inside::inner_function();
    // outermost::inside::secret_function(); // All of them will fail
}
