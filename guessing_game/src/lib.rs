pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Result<Guess, &'static str> {
        if value < 1 || value > 100 {
            return Err("The secret number will be between 1 and 100.");
        }
        Ok(Guess { value })
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
