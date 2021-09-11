use object_oriented_features::gui::{Draw, Button, Screen};
use object_oriented_features::blog::Post;

fn main() {
    let screen = Screen { // example of using trait objects
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // example of state pattern (see lib.rs -> mod blog) to accomplish the following:
    // 1. A blog post starts as an empty draft.
    // 2. When the draft is done, a review of the post is requested.
    // 3. When the post is approved twice, it gets published.
    // 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    post.add_text("!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    post.add_text("Adding text is ignored beyond draft state.");

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today!", post.content());
}


// implementation might be in a different crate
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
// Important: 'Object safety'
// A trait is object safe if all the methods defined in the trait have the following properties:

// The return type isn’t Self.
// There are no generic type parameters.