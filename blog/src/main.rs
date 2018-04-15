extern crate blog;

use blog::Post;

fn main(){
    let mut post = Post::new();
    let post_msg = "I went to work today. ";
    let post_msg2 = "And then I went to bed. ";

    post.add_text(post_msg);
    post.add_text(post_msg2);
    // Verify post content returns empty string if state is a Draft
    assert_eq!("", post.content());

    post.request_review();
    // Verify post content returns empty string if state is Request Review
    assert_eq!("", post.content());

    // This should be ignored
    post.add_text("But then I got up again. ");

    post.approve();
    // Verify post content returns only the post_msg
    assert_eq!(format!("{}{}",post_msg,post_msg2), post.content());


}