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

    pub fn request_review(&mut self) {
        self.state.replace(Box::new(PendingReview {}));
    }
}

trait State {}

struct Draft {}

impl State for Draft {}

struct PendingReview {}

impl State for PendingReview {}

struct Published {}

impl State for Published {}