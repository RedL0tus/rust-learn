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
    }
}
