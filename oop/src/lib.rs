mod encapsulation;
mod polymorphism;
mod state;

#[cfg(test)]
mod tests {
    use super::*;
    use polymorphism::{gui, Button, SelectBox};
    use state::{post::Post, rust_post::RustPost};

    #[test]
    fn polymorphism_works() {
        let screen = gui::Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ]
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ]
        };

        screen.run()
    }

    #[test]
    fn state_works() {
        let mut post = Post::new();
        
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn rust_state_works() {
        let mut post = RustPost::new();
    
        post.add_text("I ate a salad for lunch today");
    
        let mut post = post.request_review();
    
        post.approve();
        post.approve();

        let post = post.publish().unwrap();
    
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
