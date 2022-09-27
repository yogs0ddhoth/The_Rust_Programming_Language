pub enum Appetizer {
    // all variants are pub
    Soup,
    Salad,
}
pub struct Breakfast {
    // fields are private by default
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    // create an instance of Breakfast
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
pub fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}
fn cook_order() {}
