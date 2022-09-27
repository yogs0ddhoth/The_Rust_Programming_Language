#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,    
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl Coin {
    fn value_in_cents(&self) -> u8  {
        match self {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25
        }
    }
}

fn main() {
    let quarter1 = Coin::Quarter(UsState::Alabama);

    quarter1.value_in_cents();
    // if let expression is valid, execute code in block
    if let Coin::Quarter(state) = quarter1 {
        println!("State quarter from {:?}!", state);
    }
    
    let six = plus_one(Some(5));
    dbg!(six.unwrap());

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch-all for handling all unspecified values
        other => move_player(other),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // catch-all for unspecified values not used 
        _ => None,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}