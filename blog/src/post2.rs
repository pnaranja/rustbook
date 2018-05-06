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

pub struct PendingReviewPost1 {
    content: String
}

pub struct PendingReviewPost2 {
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
    pub fn add_text(&mut self, text: &str) -> &mut DraftPost {
        self.content.push_str(text);
        self
    }

    pub fn request_review(self) -> PendingReviewPost1 {
        PendingReviewPost1 {
            content: self.content
        }
    }
}

impl PendingReviewPost1 {
    pub fn approve(self) -> PendingReviewPost2 {
        PendingReviewPost2 {
            content: self.content
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}

impl PendingReviewPost2 {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}
