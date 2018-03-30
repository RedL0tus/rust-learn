extern crate publishing_a_crate_to_crates_io;

// Reexported modules can be used directly.
use publishing_a_crate_to_crates_io::PrimaryColor;

fn main() {
    let dummy_var = PrimaryColor::Red;
    println!("The dummy var is: {:?}.", dummy_var);
}
