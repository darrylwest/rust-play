use blog_post::Post;

fn main() {
    let blog_text = "this is a test blog post.";

    let mut post = Post::new();
    assert_eq!("[Draft]", post.content());
    assert_eq!("", post.draft_content());
    println!("{}: {}", post.content(), post.draft_content());

    post.add_text(blog_text);
    assert_eq!("[Draft]", post.content());
    assert_eq!(blog_text, post.draft_content());
    println!("{}: {}", post.content(), post.draft_content());

    post.request_review();

    assert_eq!("[PendingReview]", post.content());
    assert_eq!(blog_text, post.draft_content());
    println!("{}: {}", post.content(), post.draft_content());

    post.approve();

    assert_eq!(blog_text, post.content());
    assert_eq!(post.draft_content(), post.content());
    println!("Published: {}", post.content());

    post.add_text("more text added after publishing--should be restricted");
    assert_ne!(blog_text, post.content());
    // assert_eq!(post.draft_content(), post.content());
    println!("[Draft] {}", post.draft_content());
    println!("[Published] {}", post.content());
}
