/// A blog post starts as an empty draft.
/// Once the draft is done, a review of the post is requested.
/// Once the post is approved, it gets published.
/// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

use std::process;

pub struct Post{
    state: Option<Box<State>>,
    content: String
}


impl Post{
    /// A new blog post will start with a draft state
    pub fn new() -> Post{
        Post{
            state : Some (Box::new(Draft {})),
            content : String::new()
        }
    }


    /// Need a mutating self reference since content is changing
    pub fn add_text(&mut self, text : &str){
        self.content.push_str(text);
    }

    /// PlaceHolder - Return empty string for now.  Later need to check state
    pub fn content(&mut self) -> &str{
        self.state.take().unwrap_or_else(||{println!("ERROR");process::exit(1)}).content(self)
    }

    /// take() will return the Option<Box<State>> and replace self.state with None using
    /// core::mem::replace(self,None) - https://doc.rust-lang.org/core/mem/fn.replace.html
    pub fn request_review(&mut self){
        self.state = Some(self.state.take().unwrap_or_else(||{println!("ERROR");process::exit(1);}).request_review());
    }

    pub fn approve(&mut self){
        self.state = Some(self.state.take().unwrap_or_else(||{println!("ERROR");process::exit(1);}).approve());
    }

}

trait State {
    fn request_review(self : Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post)->&'a str;
}

/// Represent the Draft State
struct Draft {}

impl State for Draft {
    /// Requesting a review for a draft returns an pending review state
    fn request_review (self:Box<Self>) -> Box<State>{
        Box::new(PendingReview {})
    }

    /// Cannot approve a draft.  Needs to be reviewed
    fn approve(self: Box<Self>) -> Box<State>{ self }

    /// Unapproved content returns an empty string
    fn content<'a>(&self, post: &'a Post) -> &'a str{""}
}

/// Represent the Pending Review State
struct PendingReview {}

impl State for PendingReview {
    /// request a review for pending review state returns itself, since it's already in review!
    fn request_review (self:Box<Self>) -> Box<State>{
        self
    }

    /// Approve a pending review should return a publish state
    fn approve(self: Box<Self>) -> Box<State>{ Box::new(Publish {}) }

    /// Unapproved content returns an empty string
    fn content<'a>(&self, post: &'a Post) -> &'a str{""}
}

/// Represent the Published state.  This is when a Pending Review gets approved
struct Publish {}

impl State for Publish{
    /// requesting a review or approving a post that is already published doesn't make any sense
    fn request_review(self : Box<Self>) -> Box<State>{ self }

    /// requesting a review or approving a post that is already published doesn't make any sense
    fn approve(self : Box<Self>) -> Box<State>{ self }

    /// Return published content
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        post.content.as_ref()
    }
}
