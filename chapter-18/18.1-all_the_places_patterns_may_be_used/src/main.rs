fn main() {
    // `match` Arms
    {
        let value = 3;
        match value {
            1 => println!("The value is one."),
            2 => println!("The value is two."),
            3 => println!("The value is three."),
            4 => println!("The value is four."),
            5 => println!("The value is five."),
            _ => println!("The value is (undefined)."),
            // PATTERN => EXPRESSION,
        }
    }
    // Conditional `if let` expressions
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    // `while let` Conditional Loops
    {
        let mut stack = Vec::new();
        stack.push("怒った？？？");
        stack.push("怒った？？");
        stack.push("怒った？");
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
        println!("怒る。");
    }
    // `for` Loops
    {
        let v = vec!["怒った？", "怒った？？", "怒った？？？"];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
        println!("怒る。");
    }
    // `let` Statement
    {
        let (x, y, z) = (1, 2, 3);
        println!("x = {}, y = {}, z = {}.", x, y, z);
    }
    // Function Parameters
    {
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {}).", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
    }
}
