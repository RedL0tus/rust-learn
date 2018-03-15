enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v: Vec<i32> = Vec::new(); // Define a new empty vector
    let mut v: Vec<i32> = vec![1, 2, 3]; // Define a new vector with known values
    println!("The first value of the vector is: {:?}", v.get(0));
    v.push(4); // Push a new value to the vector
    println!("The new value of the vector is: {:?}", v.get(3));
    // println!("This line will return a panic!: {:?}", v[100]);
    // println!("This line will return a None: {:?}", v.get(100));
    
    // Invalid References
    // let mut v = vec![1, 2, 3, 4, 5];
    // let _first = &v[0];
    // v.push(6); // Won't work 

    // Iterating Over the Values in a Vector
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in &v {
        println!("{}", i);
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]; // Vectors can heve different data types by using enum
}
