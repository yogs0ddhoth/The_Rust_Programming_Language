/* State patterns are possible, if repetitive */
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            // the state of a new post will always be Some(Box<Draft>)
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            match s.state() {
                "Draft" => self.content.push_str(text),
                _ => println!("Only Drafts can be edited"),
            }
        }
    }

    pub fn content(&self) -> &str { // ownership can't be moved from &self 
        self.state
            .as_ref() // convert to a reference
            .unwrap()
            .content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    /* method is only valid on a Box holding the implementing type */
    fn request_review(
        self: Box<
            /* Self refers to the implementing type */
            Self // State
        > 
    ) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn state<'a>(self: Box<Self>) -> &'a str;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn state<'a>(self: Box<Self>) -> &'a str {
        "Draft"
    }
}

struct PendingReview {
    approvals: i32
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        match self.approvals {
            0 => {
                self.as_mut().approvals = 1;
                self
            }
            1 => {
                Box::new(Published {})
            }
            _ => self
            
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn state<'a>(self: Box<Self>) -> &'a str {
        "Pending Review"
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn state<'a>(self: Box<Self>) -> &'a str {
        "Published"
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}