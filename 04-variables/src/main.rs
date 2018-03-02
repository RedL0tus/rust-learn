fn main() {
    /* Mutability */
    let mut x = 5; // mut means mutable.
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    /* Consts */
    const EXAMPLE_CONST: u32 = 100_000; // type must be annotated
    println!("The value of the example const is: {}", EXAMPLE_CONST);

    /* Shadowing */
    let x = 5;
    let x = x + 1;
    let x = x + 2; // This is what we will see when we use it.
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len(); // Shadowing can change data type
    println!("The value of spaces is: {}", spaces);

    /*
     * let mut spaces = "     ";
     * spaces = space.len();
     * will result in an error.
     */
}
