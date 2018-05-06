/// A blog post starts as an empty draft.
/// Once the draft is done, a review of the post is requested.
/// Once the post is approved, it gets published.
/// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
///
/// Use the struct "states" to move to different states

pub struct Post2 {
    content: String
}

pub struct DraftPost {
    content: String
}

pub struct PendingReviewPost {
    content: String
}

impl Post2 {
    /// A new blog post will start with a draft state
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content
        }
    }
}

