fn main() {
    let mut x: String = String::from("Hello, world!");
    println!("{}", first_word(&x));
    println!("{},{}", second_word(&x).0, second_word(&x).1);
    println!("{}", first_word_slice(&x));
    println!("{}", second_word_slice(&x));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert String to an array of bytes.
    for (i, &item) in bytes.iter().enumerate() { // Create an iterator
        if item == b' ' { 
            return i;
        }
    }
    return s.len();
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]; // Return a slice
}

fn second_word(s: &String) -> (usize,usize) {
    let bytes = s.as_bytes();
    let mut count: usize = 0;
    let mut start: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count = count + 1;
            if count == 2 {
                return (start, i); 
            }
            start = i + 1;
        }
    }
    return (start, s.len());
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut count: usize = 0;
    let mut start: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count = count + 1;
            if count == 2 {
                return &s[start..i];
            }
            start = i + 1;
        }
    }
    return &s[start..];
}
