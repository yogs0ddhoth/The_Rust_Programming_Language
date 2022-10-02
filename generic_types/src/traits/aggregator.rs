use std::fmt::{Debug, Display};
// declare trait
pub trait Summary {
    // define methods
    // each type implementing Summary can override method behavior
    fn summarize_author(&self) -> String {
        String::from("this author")
    }
    fn summarize(&self) -> String {
        // default behavior - can call other trait methods
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}
// either the trait or the type has to be local to implement one on the other
impl Summary for Vec<&str> {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
} // implement trait
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

/* Trait Bounds */
pub fn notify(item: &impl Summary) {
    // ensure that trait methods can be called
    println!("Breaking news! {}", item.summarize());
} /* this is syntactic sugar for:
   * 
   * pub fn notify<T: Summary>(item: &T) { 
   *    // trait implementation using reference
   *    println!("Breaking news! {}", item.summarize());
   * }
   */

// impl trait allows for different types that both implement the trait
pub fn notify_1(_item1: &impl Display, _item2: &impl Display) {}

// trait bounds are necessary to force parameters to have the same type
// params in this func must share the same type implementation
pub fn notify_2<T: Display>(_item1: &T, _item2: &T) {}

// specify that param must implement multiple types with + syntax
pub fn notify_3(_item: &(impl Display + Clone)) {}

pub fn notify_3_1<T: Display + Clone>(_item: &T) {} // also valid

/* where clauses */
// note the complex type params
pub fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    1
}
// can be rewritten as:
pub fn some_where_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}