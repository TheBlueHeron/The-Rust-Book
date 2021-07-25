use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    //fn summarize(&self) -> String; // all implementors múst implement this method
    fn summarize(&self) -> String {  // trait method with default behavior
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    //fn summarize(&self) -> String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    //} // if not implemented, the default behavior will be used!
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}
// Using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { // new is always implemented
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // cmp_display is implemented only if T implements Display and PartialOrd
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x: {}", self.x);
        } else {
            println!("The largest member is y: {}", self.y);
        }
    }
}