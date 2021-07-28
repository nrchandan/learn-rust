
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..101).contains(&value) {
            panic!("Guess value must be between 1 and 100. Got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
