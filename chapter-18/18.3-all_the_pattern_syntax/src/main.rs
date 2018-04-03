fn main() {
    // Matching Literals
    {
        let x = 1;
        match x {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            _ => println!("(undefined)"),
        }
    }
    // Matching Named Variables
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}.", y),
            _ => println!("Default case, x = {:?}.", x),
        }
        println!("At the end: x = {:?}, y = {:?}.", x, y);
    }
    // Multiple Patterns
    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("Three"),
            _ => println!("(undefined)"),
        }
    }
    // Matching Ranges of Values with `...`
    {
        let x = 5;
        match x {
            1 ... 5 => println!("One through five"),
            _ => println!("Something else"),
        }
        
        // Matching using `...` for chat values
        let x = 'c';
        match x {
            'a' ... 'j' => println!("Early ASCII letter"),
            'k' ... 'z' => println!("Late ASCII letter"),
            _ => println!("(undefined)"),
        }
    }
    // Destructuring to Break Apart Values
    {
        // Destructuring Structs
        {
            struct Point {
                x: i32,
                y: i32,
            }

            let p = Point { x: 0, y: 7 };

            let Point { x: a, y: b } = p;
            println!("a = {:?}, should be 0.", a);
            println!("b = {:?}, should be 7.", b);
        }
        // Destructuring Enums
        {
            #[allow(dead_code)]
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }
            let msg = Message::ChangeColor(0, 160, 255);

            match msg {
                Message::Quit => {
                    println!("The Quit variant has no data to destructure.")
                },
                Message::Move { x, y } => {
                    println!(
                        "Move in the x direction {} and in the y direction {}",
                        x,
                        y
                    );
                }
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!(
                        "Change the color to red {}, green {}, and blue {}",
                        r,
                        g,
                        b
                    )
                }
            }
        }
        struct Point { x: i32, y: i32 }; // Needed for the next two parts
        // Destructuring References
        {
            let points = vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 5 },
                Point { x: 10, y: -3 },
            ];

            let sum_of_squares: i32 = points
                .iter()
                .map(|&Point { x, y }| x * x + y * y)
                .sum();
            println!("The sum is: {}.", sum_of_squares);
        }
        // Destructing Structs and Tuples
        {
            let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
            println!("Feet = {}, inches = {}, the point is at x = {}, y = {}.", feet, inches, x, y);
        }
    }
    // Ignoring Values in a Pattern
    {
        // Ignoring an Entire Value with `_`
        {
            fn foo(_: i32, y: i32) {
                println!("This code only uses the y parameter: {}.", y);
            }

            foo(3, 4);
        }
        // Ignoring Parts of a Value with a Nested `_`
        {
            let mut setting_value = Some(5);
            let new_setting_value = Some(10);

            match (setting_value, new_setting_value) {
                (Some(_), Some(_)) => {
                    println!("Can't overwrite an existing customized value");
                }
                _ => {
                    setting_value = new_setting_value;
                }
            }

            println!("setting is {:?}", setting_value);
        }
        // Ignoring an Unused Variable by Starting its name with an Underscore
        {
            let _x = Some(String::from("Hello!")); // It will be ignored.
            if let Some(_s) = _x {
                println!("Found a string: {:?}", _s);
            }
        }
        // Ignoring Remaining Parts of a Value with `..`
        {
            #[allow(dead_code)]
            struct Point {
                x: i32,
                y: i32,
                z: i32,
            }
            let origin = Point { x: 0, y: 0, z: 0 };
            match origin {
                Point { x, .. } => println!("x = {}", x),
            }

            let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024);
            match numbers {
                (first, .., last) => {
                    println!("Some numbers: {}, {}", first, last);
                }
            }
        }
    }
    // `ref` and `ref mut` to Create References in Patterns
    {
        let mut robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }
        match robot_name {
            Some(ref mut name) => *name = String::from("Yet another name"),
            None => (),
        }
        println!("Robot name is: {:?}.", robot_name);
    }
    // Extra Conditionals with Match Guards
    {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("Less than five: {}", x),
            Some(x) => println!("{}", x),
            _ => (),
        }

        // Combining multiple patterns with a match guard
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("Yay!"), // Equal to `(4 | 5 | 6) if y`
            _ => println!("Nan."),
        }
    }
    // `@` Bindings
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_variable @ 3...7 } => { // Use `@` to bind a value in a pattern while also tesing it
                println!("Found an id in range: {}", id_variable)
            },
            Message::Hello { id: 10...12 } => {
                println!("Found an id in another range")
            },
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            },
        }
    }
}
