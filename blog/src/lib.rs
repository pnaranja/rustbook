/// A blog post starts as an empty draft.
/// Once the draft is done, a review of the post is requested.
/// Once the post is approved, it gets published.
/// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

use std::process;

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    /// A new blog post will start with a draft state
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }


    /// Add text to content
    /// Need a mutating self reference since content is changing
    pub fn add_text(&mut self, text: &str) {
        self.content =
            self.state.as_ref() // Since self is a ref, the contents of self needs to be a ref?
                .unwrap_or_else(|| {
                    println!("ERROR attempting to add text");
                    process::exit(1)
                })
                .check_text(self, text);
    }

    /// Returns content dependant on the state
    pub fn content(&self) -> &str {
        self.state.as_ref() // Since self is a ref, the contents of self needs to be a ref?
            .unwrap_or_else(|| {
                println!("ERROR retrieving content");
                process::exit(1)
            }).content(self)
    }

    /// Put Draft state to Pending Review
    /// take() will return the Option<Box<State>> and replace self.state with None using
    /// core::mem::replace(self,None) - https://doc.rust-lang.org/core/mem/fn.replace.html
    pub fn request_review(&mut self) {
        self.state = Some(self.state.take()
            .unwrap_or_else(|| {
                println!("ERROR requesting review");
                process::exit(1);
            })
            .request_review());
    }

    /// Approve content and set state to Publish if it's been reviewed
    pub fn approve(&mut self) {
        self.state = Some(self.state.take()
            .unwrap_or_else(|| {
                println!("ERROR attempting to approve");
                process::exit(1);
            })
            .approve());
    }

    /// Reject content and set state to Draft if it's been reviewed
    pub fn reject(&mut self) {
        self.state = Some(self.state.take()
            .unwrap_or_else(|| {
                println!("ERROR attempting to approve");
                process::exit(1);
            })
            .reject());
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn reject(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
    fn check_text<'a>(&self, post: &'a Post, new_content: &'a str) -> String;
}

/// Represent the Draft State
struct Draft {}

impl State for Draft {
    /// Requesting a review for a draft returns an pending review state
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview { previous_approvals: 0 })
    }

    /// Cannot approve a draft.  Needs to be reviewed
    fn approve(self: Box<Self>) -> Box<State> { self }

    /// Cannot reject a draft.  Needs to be reviewed
    fn reject(self: Box<Self>) -> Box<State> { self }

    /// Unapproved content returns an empty string
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }

    /// Add text if in a Draft state
    fn check_text<'a>(&self, post: &'a Post, new_content: &'a str) -> String {
        format!("{}{}", post.content.clone(), new_content)
    }
}

/// Represent the Pending Review State
struct PendingReview {
    previous_approvals: i8
}

impl State for PendingReview {
    /// request a review for pending review state returns itself, since it's already in review!
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    /// Approve a pending review should return a publish state
    fn approve(self: Box<Self>) -> Box<State> {
        if self.previous_approvals == 1 {
            Box::new(Publish {})
        } else {
            Box::new(PendingReview { previous_approvals: self.previous_approvals + 1 })
        }
    }

    /// Reject a pending review should go back to a draft
    fn reject(self: Box<Self>) -> Box<State> { Box::new(Draft {}) }

    /// Unapproved content returns an empty string
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }

    /// Can only add more text if in a Draft state.  Just return the post's original content
    fn check_text<'a>(&self, post: &'a Post, _new_content: &'a str) -> String { post.content.clone() }
}

/// Represent the Published state.  This is when a Pending Review gets approved
struct Publish {}

impl State for Publish {
    /// requesting a review or approving a post that is already published doesn't make any sense
    fn request_review(self: Box<Self>) -> Box<State> { self }

    /// requesting a review or approving a post that is already published doesn't make any sense
    fn approve(self: Box<Self>) -> Box<State> { self }

    /// Rejecting a post that is already published doesn't make any sense
    fn reject(self: Box<Self>) -> Box<State> { self }

    /// Return published content
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        post.content.as_ref()
    }

    /// Can only add more text if in a Draft state.  Just return the post's original content
    fn check_text<'a>(&self, post: &'a Post, _new_content: &'a str) -> String { post.content.clone() }
}
