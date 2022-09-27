// bring module files into scope
pub mod back_of_house;
pub mod front_of_house;

// Refer to a module using its path
use front_of_house::hosting; // Relative Path
                             // Absolute Path crate::front_of_house::hosting::seat_at_table;
use back_of_house::Breakfast;

// 'pub' re-exports these modules to parent scopes
pub use std::{
    // imports in the same scope can be destructured
    fmt::Result,
    io::{
        self,               // the parent scope can be included in the destructured imports as 'self'
        Result as IoResult, // Type aliases are needed when multiple imported types use the same name
    },
};

pub mod customer {
    pub fn eat_at_restaurant() {
        // call functions from a module in the parent scope
        super::hosting::add_to_waitlist();
        super::hosting::seat_at_table();

        // create instance of a Breakfast with "rye" toast
        let mut meal = super::Breakfast::summer("rye");

        // update meal <- possible because 'toast' is a pub field
        meal.toast = String::from("wheat");
        println!("I'd like {} toast, please", meal.toast);
    }
}
fn deliver_order() {}
