pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new(state: Option<Box<dyn State>>) -> Post {
        Post {
            state,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        if self.state.as_ref().unwrap().can_get_content() {
            self.content.as_str()
        } else {
            ""
        }
    }

    // this fn was used when states were in this file
    // pub fn content(&self) -> &str {
    //     self.state.as_ref().unwrap().content(self)
    // }

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

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn can_get_content(&self) -> bool {
        false
    }
}
