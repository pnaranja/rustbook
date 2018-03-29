// A blog post starts as an empty draft.
// Once the draft is done, a review of the post is requested.
// Once the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

use std::process;

pub struct Post{
    state: Option<Box<State>>,
    content: String
}

impl Post{
    fn new() -> Post{
        // A new blog post will start with a draft state
        Post{
            state : Some (Box::new(Draft {})),
            content : String::new()
        }
    }

    // Need a mutating self reference since content is changing
    pub fn add_text(&mut self, text : &str){
        self.content.push_str(text);
    }

    // PlaceHolder - Return empty string for now.  Later need to check state
    pub fn content(&self) -> &str{
        ""
    }

    // take() will return the Option<Box<State>> and replace self.state with None using
    // core::mem::replace(self,None) - https://doc.rust-lang.org/core/mem/fn.replace.html
    pub fn request_review(&mut self){
        self.state = Some(self.state.take().unwrap_or_else(||process::exit(1)).request_review());
    }

}

trait State {
    fn request_review(self : Box<Self>) -> Box<State>;
}

// Represent the Draft State
struct Draft {}

impl State for Draft {
    // Requesting a review for a draft returns an pending review state
    fn request_review (self:Box<Self>) -> Box<State>{
        Box::new(PendingReview {})
    }
}

// Represent the Pending Review State
struct PendingReview {}

impl State for PendingReview {
    // For now? request a review for pending review state returns itself
    fn request_review (self:Box<Self>) -> Box<State>{
        self
    }
}
