use std::ops::Add;
/* Rust allows for overloading operations and corresponding traits listed in the std::ops by implementing traits associated with the operator */
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/* definition of Add in the standard library for reference */
trait _Add<Rhs = Self> {
    // Rhs=Self is the default type, giving greater flexibility to the generic type param
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, PartialEq)]
pub struct Meters(pub u32);

#[derive(Debug, PartialEq)]
pub struct Millimeters(pub u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
