#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) || 
        (self.height > other.width && self.width > other.height)
    }
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100."
            );
        }

        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/* Unit Tests */
#[cfg(test)] // only compile this code if tests are being run
mod tests {
    use super::*;

    // #[test] functions will be run on parallel threads

    // cargo test {string} <- run all test functions whose names match the string
    // cargo test -- --test-threads={number} <- specify how many test threads to use

    // cargo test -- --show-output <- will show printed values

    // cargo test -- --ignored <- run only ignored tests
    // cargo test -- --include-ignored <- run all tests, including ignored

    #[test] // annotate test function
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn larger_can_hold() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore] // exclude this test from default run
    #[should_panic(
        expected = "Guess value must be less than or equal to 100." // expected panic message
    )] // function should panic
    fn greater_than_100() {
        Guess::new(200);
    }
}
