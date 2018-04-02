fn main() {
    // let Some(x) = Some(7); // Will fail

    if let Some(x) = Some(7) {
        println!("{}", x);
    }

    // The code below will fail, too.
    // if let x = 5 {
    //     println!("{}", x);
    // }
}
