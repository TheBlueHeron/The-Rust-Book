fn main() {
    let mut s = String::from("hello world");

    let _word = first_word_1(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");
    
    let _hello = &s[0..5];
    let _world = &s[6..11];
    let _slice = &s[0..2];
    let _slice = &s[..2]; // this is equal to the previous assignment (using ASCII!)
    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..]; // this is equal to the previous assignment (using ASCII!)
    let _slice = &s[0..len];
    let _slice = &s[..]; // this is equal to the previous assignment (using ASCII!)

    let mut s = String::from("hello world");

    let word = first_word_2(&s);

    s.clear(); // error! -> mutable borrow áfter immutable borrow

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let _word = first_word_3(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word_3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_3(my_string_literal);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3]; // type is here &[i32], see also 'vectors' (chapter 8)
}

fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // s itself was also passed as a reference
}

fn first_word_3(s: &str) -> &str { // this allows us to use the same function on both &String values and &str values
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}