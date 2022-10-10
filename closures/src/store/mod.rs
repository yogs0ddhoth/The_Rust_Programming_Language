#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    pub fn new(shirts: Vec<ShirtColor>) -> Inventory {
        Inventory { shirts }
    }
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(
            // closure captures an immutable reference to self and passes it
            |/* params */| 
            self.most_stocked(), // body calls method on self
        )
    }
    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
