#[derive(Default)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
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
    fn name(self: Box<Self>) -> String;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(self: Box<Self>) -> String {
        "Draft".to_string()
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn name(self: Box<Self>) -> String {
        "PendingReview".to_string()
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(self: Box<Self>) -> String {
        "Published".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post() {
        let mut post = Post::new();

        assert_eq!("", post.content());

        post.add_text("i need water quick.");
        assert_eq!("", post.content());
        assert_ne!("", post.content);

        // println!("state 1: {}", post);

        post.request_review();

        assert_eq!("", post.content());
        assert_ne!("", post.content);

        // println!("state 2: {}", post.show_state());
        
    }
}
