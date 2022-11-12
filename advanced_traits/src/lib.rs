mod associated_types;
mod newtype;
mod operator_overloading;
mod qualified_syntax;
mod supertraits;

#[cfg(test)]
mod tests {
    use super::*;
    use associated_types::{Counter, Iterator};
    use core::fmt;
    use newtype::Wrapper;
    use operator_overloading::{Meters, Millimeters, Point};
    use qualified_syntax::{Animal, Dog, Human, Pilot, Wizard};
    use supertraits::OutlinePrint;

    #[test]
    fn associated_types_work() {
        let mut counter = Counter {};
        assert_eq!(0, counter.next().unwrap());
    }

    #[test]
    fn operator_overloading_works() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
        assert_eq!(Millimeters(1500), Millimeters(500) + Meters(1))
    }

    #[test]
    fn qualified_syntax_works() {
        /* <Type as Trait>::function(reciever_if_method, next_arg, ...) */

        // Different levels of qualification are required for functions with the same name implemented on the same type, depending on the function:

        /* traits methods can be called off the trait, passing the type that implements the trait */
        let person = Human;
        Pilot::fly(&person); // call the fly() method on the Pilot trait implemented for Human
        <Human as Wizard>::fly(&person); // fully qualified syntax specifies which implementation the method comes from, and isn't necessary for trait methods
        person.fly(); // the default fly() method will be the one implemented on the type

        /* associated functions that aren't methods (don't have a self parameter) require fully qualified synjtax */
        println!("\nA baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    #[test]
    fn supertraits_work() {
        struct Point {
            x: i32,
            y: i32,
        }
        // satisfy the fmt::Display supertrait requirement
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        impl OutlinePrint for Point {}

        OutlinePrint::outline_print(&Point { x: 0, y: 0 });
    }

    #[test]
    fn newtype_works() {
        let w = Wrapper(vec![String::from("Hiya"), String::from("Chuck")]);
        println!("w = {}", w);
    }
}
