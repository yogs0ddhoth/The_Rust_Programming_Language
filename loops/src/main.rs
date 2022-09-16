fn main() {
    loops();

    let (mut t, mut u) = (31.0, 'C');
    println!("{t}° {u}");

    (t, u) = convert_temp(&t, u);
    println!("converts to {t}° {u}\n");

    println!("The fifth number in the fibonacci sequence is {}\n", fibonacci_n(5));

    twelve_days_of_christmas_1780();
}

fn loops() {
        /* Eternal Loop
      loop { // must be manually stopped -> ctrl+c
          println!("again!");
      }
    */

    /* Basic Loop */
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            /* 'break' keyword creates a break expression <- remember expressions return values */
            break counter * 2;
        }
    };
    println!("The result is {result}\n");

    /* Nested Loop */
    let mut count = 0;

    'counting_up: loop { // loop through count, 0 to 2
        println!("count = {count}");
        let mut remaining = 10;

        loop { // loop through remaining, 10 to 9
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}\n");

    /* While */
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!\n");

    /* For */
    let a = [5, 4, 3, 2, 1];

    for number in a {
        println!("{number}!");
    }
    println!("LIFTOFF!!\n");

    /* Range -> for definite loops */
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!\n");
}

/// Converts a temperature between Fahrenheit and Celsius
/// # Arguments
/// * temp - pointer to in i32 integer
/// * unit - 'C'|'F' char corresponding to the unit of temp, defaults to 'F'
fn convert_temp(temp: &f64, unit: char) -> (f64, char) {

    return match unit {
        'C' => (*temp * 1.8 + 32.0, 'F'),
        'F' | _ => ((*temp - 32.0) / 1.8, 'C'),
    };
}

/// returns the nth number in the fibonacci sequence using the formula:
/// 
/// Fn = (φ^n - (1-φ)^n)/√5; where φ = (1+√5)/2
fn fibonacci_n(n:i32) -> i32 {
    let sqrt_five = f64::sqrt(5.0);
    let golden_ratio = (1.0 + sqrt_five)/ 2.0;

    (
        (
            golden_ratio.powi(n) - (1.0-golden_ratio).powi(n)
        )/sqrt_five
    ) as i32
}

/// Prints the lyrics to The 'Twelve Days of Christmas,' using the version from the 1780 book: "Mirth without Mischief"
fn twelve_days_of_christmas_1780() {
    fn format_day(n: i32) -> String {
        return match n {
            1 => format!("{}st", n),
            2 => format!("{}nd", n),
            3 => format!("{}rd", n),
            4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | _ => format!("{}th", n),
        };
    }
    fn add_gifts(n: i32) -> &'static str {
        return match n {
            1 => "a partridge in a pear-tree",
            2 => "two turtle doves,",
            3 => "three french hens,",
            4 => "four colly birds,",
            5 => "five gold rings,",
            6 => "six geese a laying,",
            7 => "seven swans a swimming,",
            8 => "eight maids a milking,",
            9 => "nine drummers drumming,",
            10 => "ten pipers piping,",
            11 => "eleven ladies dancing,",
            12 => "twelve lords a leaping,",
            _ => "an error.",
        }
    }

    let mut gifts: String = "".to_string();

    for number in 1..=12 {
        gifts = match number {
            1 => format!("{}", add_gifts(number)),
            2 => format!("{} and {}", add_gifts(number), gifts),
            _ => format!("{} {}", add_gifts(number), gifts),
        };
        println!("On the {} day of Christmas, my true love sent to me: {}!", format_day(number), gifts)
    }
}