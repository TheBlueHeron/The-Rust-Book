
pub struct AveragedCollection {// example of 'encapsulation'
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection { // caches the average
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 { // average is read-only
        self.average
    }

    fn update_average(&mut self) { // private
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Inheritance: --> default trait method implementations
// Polymorphism: -> generics and trait bounds
//                  (bounded parametric polymorphism)

pub mod gui {

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // a vector of trait objects that implement the Draw trait
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// implementation using generics and trait bounds: 'components' can hold only one type (T)
// this is preferable only in the case of a homogenous collection, because the definition will be 'monomorphized' at compile time
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}



}

pub mod blog {

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
            self.content.push_str(self.state.as_ref().unwrap().add_text(text));
        }
        // The following methods know nothing about the various behaviors!
        pub fn content(&self) -> &str {
            // don't take ownership of value
            // unwrap it, because an Option<&Box<dyn State>> is returned
            // A None is never returned
            // deref coercion will take effect on the & and the Box,
            // so the content method will ultimately be called on the type that implements the State trait
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
        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    trait State {
    // This syntax means the method is only valid when called on a Box holding the type.
    // This syntax takes ownership of Box<Self>, invalidating the old state,
    // so the state value of the Post can transform into a new state.
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            "" // default implementation (Draft, PendingReview) returns an empty string
        }
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn add_text<'a>(&self, text: &'a str) -> &'a str {
            "" // only overridden in Draft implementation of State
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview::new(false))
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self // can't change state from Draft to Published
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self // already a Draft
        }
        fn add_text<'a>(&self, text: &'a str) -> &'a str {
            text // text may be added, so return it
        }
    }

    struct PendingReview {
        reviewed: bool  // remember previous approval
    }

    impl PendingReview{
        pub fn new(ok: bool) -> PendingReview {
            PendingReview {
                reviewed: ok
            }
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self // it returns itself, because when we request a review on a post already in the PendingReview state,
                 // it should stay in the PendingReview state. I.e each state is responsible for its own rules!
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            if self.reviewed {
                Box::new(Published {}) // change to Published state
            } else { // 1st approval, remember it
                Box::new(PendingReview::new(true))
            }
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft{}) // change back to Draft state
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self // already published
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self // already published
        }
        // override the content method and return the value in post.content
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self // already Published
        }
    }
}