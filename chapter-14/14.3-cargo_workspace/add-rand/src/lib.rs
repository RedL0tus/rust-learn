extern crate rand;

use rand::Rng;

pub fn add_rand(x: i32) -> i32 {
    return x + rand::thread_rng().gen_range(1, 101);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
