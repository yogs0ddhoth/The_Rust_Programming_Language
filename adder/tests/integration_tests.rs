/* Integration Tests */

use adder;
// each file will be compiled as its own seperate crate, so modules must be seperated into directories
mod common;

// will only run if all unit tests pass
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add(2, 2));
}