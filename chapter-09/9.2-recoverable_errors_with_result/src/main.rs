use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // Use match
    let _result = read_file_match(String::from("test-0.txt"));
    // Shortcuts for Panic on Error: `unwrap` and `expect`.
    let _result = read_file_direct_panic(String::from("test-1.txt"));
    // Propagating Errors
    let _result = read_file_return_result(String::from("test-2.txt"));
    println!("The result of last line is: {:?}.", _result);
    // A shortcut for Propagating Errors: ?
    let _result = read_file_return_result_shortcut(String::from("test-3.txt"));
    println!("The result of last line is: {:?}.", _result);
}

fn read_file_match(filename: String) -> std::fs::File {
    let f = File::open(&filename);

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // Handle different kinds of errors in different ways
            match File::create(&filename) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
    return f;
}

fn read_file_direct_panic(filename: String) -> std::fs::File {
    let f = File::open(&filename).expect("Failed to open file");
    // or
    // let f = File::open("hello.txt").unwrap();
    return f;
}

fn read_file_return_result(filename: String) -> Result<String, io::Error> {
    let f = File::open(&filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file_return_result_shortcut(filename: String) -> Result<String, io::Error> {
    let mut s = String::new();
    // ? can only be used in functions that return result.
    File::open(&filename)?.read_to_string(&mut s)?;
    return Ok(s);
}


