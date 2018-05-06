//extern crate blog;
//
//use blog::Post;

mod post;
mod post2;

fn main() {
    test_post();
    test_post2()
}

fn test_post2() {
    let mut post = post2::Post2::new();
    let post_msg = "I went to work today. ";
    let post_msg2 = "And then I went to bed. ";
    let post_msg3 = "And had a good sleep. ";

    post.add_text(post_msg).add_text(post_msg2).add_text(post_msg3);

    let rejected_post = post.request_review().reject();
    let approved_post = rejected_post.request_review().approve();
    println!("{}", approved_post.content());
}

fn test_post() {
    let mut post = post::Post::new();
    let post_msg = "I went to work today. ";
    let post_msg2 = "And then I went to bed. ";
    let post_msg3 = "And had a good sleep. ";

    post.add_text(post_msg);
    post.add_text(post_msg2);
    // Verify post content returns empty string if state is a Draft
    assert_eq!("", post.content());

    post.request_review();
    // Verify post content returns empty string if state is Request Review
    assert_eq!("", post.content());

    post.reject();

    // Should be able to add more text because state should be a draft
    post.add_text(post_msg3);
    post.request_review();

    // This should be ignored because not a draft anymore
    post.add_text("But then I got up again. ");

    post.approve(); // Need 2 approvals to publish
    // Verify post content returns only the post_msg
    assert_eq!("", post.content());

    post.reject();
    post.request_review();

    post.approve(); // Lost the last review.  Still need 2 approvals
    assert_eq!("", post.content());

    post.approve();
    // Verify post content returns only the post_msg
    assert_eq!(format!("{}{}{}", post_msg, post_msg2, post_msg3), post.content());
}