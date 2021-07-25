use generics_traits::{self, Tweet, NewsArticle, Summary};

fn main() {
    let tweet = Tweet {
        username: String::from("Michael"),
        content: String::from(
            "Tout vient à point a çelui qui sait attendre.",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

pub fn notify(item: &impl Summary) { // a Trait as a parameter type; &impl is syntax sugar for the longer form below
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item: &(impl Summary + Display)) { multiple trait bounds; same as below
// pub fn notify<T: Summary + Display>(item: &T) {

// Clearer trait bounds with where clauses
// Default: fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// Better : fn some_function<T, U>(t: &T, u: &U) -> i32
//              where T: Display + Clone,
//                    U: Clone + Debug
//          {...

fn returns_summarizable() -> impl Summary { // return a type that implements a trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}