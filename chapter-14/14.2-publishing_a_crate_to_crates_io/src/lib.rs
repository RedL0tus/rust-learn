//! # Yet Another Example Crate
//!
//! `publishing_a_crate_to_crates_io` is a example crate from Rust book 2nd edition  
//! Here is a example of commenting contained items.

/// Adds one to the number given.  
/// Below is an example of comments as tests.
///
/// # Examples
///
/// ```
/// let five = 5;
/// assert_eq!(publishing_a_crate_to_crates_io::add_one(5), 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

// Exporting a convenient public API with `pub use`
pub mod kinds {
    /// The primary colors according to the PYR color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}

pub use kinds::PrimaryColor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn plus_one() {
        assert_eq!(add_one(5), 6);
    }
}
