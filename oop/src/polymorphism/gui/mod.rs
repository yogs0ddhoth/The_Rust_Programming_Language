/* Traits allow for sharing abstractions of common behavior */
pub trait Draw {
    fn draw(&self);
}

/* Trait implementations as a type allow for polymorphism */
pub struct Screen {
    // dyn Draw means T implements Draw
    pub components: Vec<Box<dyn Draw>>,

    // This is more flexible than using a generic type parameter with trait bounds 
    // This allows for homogenous collections, with a runtime cost
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}