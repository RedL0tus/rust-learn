pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("Yay!");
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

use a::series::of;

pub mod tests {
    use super::a::series::of; // Use super to access parent module
    
    pub fn it_works() {
        of::nested_modules();
    }
}

fn main() {
    of::nested_modules();
    let _red = Red;
    let _yellow = Yellow;
    let _green = Green;
    tests::it_works();
}
