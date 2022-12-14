use std::fmt::Display;

pub mod traits;
pub mod lifetimes;

// struct uses generic type params
// works the same on enums -> Result<T, E>
pub struct Point<T, F> {
    x: T,
    y: T,
    z: T,
    t: F,
}
// methods with generic types
impl<T, F> Point<T, F> {
    // generic constructor
    pub fn new(x: T, y: T, z: T, t: F) -> Self {
        Self { x, y, z, t }
    }
    // generic getters
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &T {
        &self.y
    }
    pub fn z(&self) -> &T {
        &self.z
    }
    pub fn t(&self) -> &F {
        &self.t
    }

    // generic types used in params
    pub fn mixup<H, C>(self, other: Point<H, C>) -> Point<T, C> {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
            t: other.t,
        }
    }
}
// type specific methods
impl<F> Point<f64, F> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
// generic type T must implement traits: Display and PartialOrd
impl<T: Display + PartialOrd, F>Point<T, F> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// generic type params
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
