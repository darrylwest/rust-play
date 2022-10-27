// Ref: Rust Book: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
//
// This implementation, from the book, uses traits to mask the blog's content until it's published.  I think
// a better approach would be to use enum's for Draft, Pending, and Publish.
// To expose pre-published content for viewing by the approver I added a func called draft_content to return
// a copy of the blog text.

#[derive(Default)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn draft_content(&self) -> String {
        self.content.to_string()
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
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
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

// --snip--
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "[Draft]"
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "[PendingReview]"
    }
}

struct Published {}

impl State for Published {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post() {
        let blog_text = "this is a test blog post.";

        let mut post = Post::new();
        assert_eq!("[Draft]", post.content());
        assert_eq!("", post.content);

        post.add_text(blog_text);
        assert_eq!("[Draft]", post.content());
        assert_eq!(blog_text, post.content);
        assert_eq!(post.draft_content(), post.content);

        post.request_review();

        assert_eq!("[PendingReview]", post.content());
        assert_eq!(blog_text, post.content);
        assert_eq!(post.draft_content(), post.content);

        post.approve();

        assert_eq!(blog_text, post.content());
        assert_eq!(post.draft_content(), post.content);
        println!("{}", post.content());

        post.add_text("more text added after publishing--should be restricted");
        assert_ne!(blog_text, post.content());

        println!("{}", post.content());
    }
}
