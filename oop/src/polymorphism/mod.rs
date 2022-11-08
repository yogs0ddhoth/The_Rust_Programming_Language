pub mod gui;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl gui::Draw for Button {
    fn draw(&self) {
        // draw button
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl gui::Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}