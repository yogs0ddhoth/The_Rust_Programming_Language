/* PATTERNS can be refutable - can fail to match - and irrefutable - can't fail */

// A Pattern consists of:
// * Literals
// * Destructured - using let PATTERN = EXPRESSION
//     * Arrays
//     * Enums
//     * Structs
//     * Tuples
// * Vairables - using let PATTERN = EXPRESSION
// * Wildcards
// * Placeholders

// **patterns can also be function params

#[cfg(test)]
mod tests {
    use super::*;

    /* Function parameters, let statements, and for loops only accept irrefutable patterns */
    // matching, conditional logic, and loops all follow a similar PATTERN => EXPRESSION
    #[test]
    fn match_works() {
        let x = Some(1);
        /* Match is exhaustive and must account for all possible patterns:
         * Normal Match arms must use refutable PATTERN => EXPRESSIONs that either account for ALL patterns
         * OR a final irrefutable PATTERN => EXPRESSION catchall can be used
         * */
        //
        let y = match x {
            // an if condition, i.e a Match Guard, can further refine a match expression at the expense of exhaustiveness
            Some(i) if (i % 2 != 0) => Some(i + 1),
            _ => None,
        };
        assert_eq!(2, y.unwrap());

        // because match starts a new scope, variables declared as part of the pattern will shadow those outside the match construct
        let x = Some(5);
        let y = 10;

        match x {
            // match expressions can match multiple patterns
            Some(50) | Some(10) => println!("Got either 50 or 10"),
            Some(y) => println!("Matched, y = {y}"), // new y variable
            _ => println!("Default case, x = {:?}", x),
            // new y dropped at the end of
        }
        println!("at the end: x = {:?}, y = {y}", x);

        // ..= allows a range pattern to be matched
        let c = 'c';
        let q = false;
        match c {
            // match guards can be useful to block off ranges
            'a'..='j' if q => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }

        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        match p {
            // Rust supports destructuring structs as a pattern to match
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }

        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        // enums and structs and tuples can be nested and stil destructured
        // values can be ignored with '_' or '..' <- for first/last
        let ((_, inches), Point { x, .. }, msg) = (
            (3, 0),
            Point { x: 3, y: -10 },
            Message::ChangeColor(Color::Hsv(0, 160, 255)),
        );

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),

            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
        }

        {
            enum Message {
                Hello { id: i32 },
            }
        
            let msg = Message::Hello { id: 5 };
        
            match msg {
                Message::Hello {
                    // the @ allows creation of a variable in a range
                    id: id_variable @ 3..=7,
                } => println!("Found an id in range: {}", id_variable),
                Message::Hello { id: 10..=12 } => {
                    println!("Found an id in another range")
                }
                Message::Hello { id } => println!("Found some other id: {}", id),
            }
        }
    }

    #[test]
    fn else_if_let_works() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        // if/if let and else if/else if let Expressions allow for complex conditional flows but aren't exhaustive and can let logic bugs slip by
        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
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

    #[test]
    fn while_works() {
        let mut stack = vec![1, 2, 3];

        // while let allows for conditional loops and destructuring
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

        stack = vec![1, 2, 3];

        // for loops allow similar features for iterables
        for (index, value) in stack.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
}
