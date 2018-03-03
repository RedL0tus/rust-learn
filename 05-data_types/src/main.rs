fn main() {
    // Scalar Types
    
    // Integer Types
    // isize: Signed integer.
    // usize: Unsigned ineger.
    let x: i16 = 0b1111_0000; // Signed 16 bit; Binary
    println!("The value of x is: {}", x);
    let x: u8 = b'A'; // Unsigned 8 bit; Byte
    println!("The value of x is: {}", x);
    let x: u32 = 98_222; // Deimal
    println!("The value of x is: {}", x);
    let x: u8 = 0xff; // Hex
    println!("The value of x is: {}", x);
    let x: u8 = 0o77; // Octal
    println!("The value of x is: {}", x);

    // Floating-Point Types
    // f32 & f64, f64 is the default
    let x = 2.0; // f64
    println!("The value of x is: {}", x);
    let x: f32 = 3.0; //f32
    println!("The value of x is: {}", x);

    // Numeric Operations
    let x: u16 = 5 + 10; // Addition
    println!("The value of x is: {}", x);
    let x: f32 = 95.5 - 4.3; // Substraction
    println!("The value of x is: {}", x);
    let x: u16 = 4 * 30; // Multiplication
    println!("The value of x is: {}", x);
    let x: f32 = 56.7 / 32.2; // Divition
    println!("The value of x is: {}", x);
    let x: u16 = 43 % 5; // Remainder
    println!("The value of x is: {}", x);

    // Boolean Type
    let x: bool = true;
    println!("The value of x is: {}", x);

    // Character Type
    let x: char = 'z';
    println!("The value of x is: {}", x);
    let x: char = 'ℤ';
    println!("The value of x is: {}", x);
    let x: char = '🌝';
    println!("The value of x is: {}", x);

    // Compound Types
    // Grouping Values into Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of x.0 is: {}", x.0);

    // Arrays
    let x = [1, 2, 3, 4, 5];
    println!("The value of x[0] is: {}", x[0]);

    let x = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
    println!("The value of s[0] is: {}", x[0]);
}
