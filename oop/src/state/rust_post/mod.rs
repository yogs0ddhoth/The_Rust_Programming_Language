/* Implementation of state concepts using Idiomatic Rust 
 *
 * State is now is now abstracted as types
 */
pub struct RustPost {
    content: String
}
impl RustPost {
    // RustPost::new() creates intermediary type
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() } 
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

/* Intermediary Types: 
 *
 * pass ownership of self from type to type
 */
pub struct DraftPost {
    content: String // field encapsulated, with no getter
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            approvals: 0,
            content: self.content
        }
    }
}

// Implementing an approval count is trickier without trait implementation for polymorphism
pub struct PendingReviewPost {
    approvals: i32, 
    content: String,
}
impl PendingReviewPost {
    pub fn approve(&mut self) {
        match self.approvals {
            0 => self.approvals += 1,
            1 => {
                self.approvals += 1;
                println!("Post is ready for approval");
            }
            _ => println!("Post is ready for approval")
        }
    }
    pub fn publish(self) -> Result<RustPost, String> {
        match self.approvals {
            2 => Ok(RustPost {content: self.content}),
            _ => Err(String::from("Post is not approved"))
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}