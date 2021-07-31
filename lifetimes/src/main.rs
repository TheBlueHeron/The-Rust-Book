fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let s: &'static str = "I have a static lifetime."; // this string is stored directly in the program’s binary, which is always available

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part)
}

// fn longest(x: &str, y: &str) -> &str { // no lifetime known
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // input parameters must have lifetime at least as long as lifetime 'a
    if x.len() > y.len() {                          // output parameter lifetime must match lifetime of óne of the input parameters
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> { // lifetime annotation in struct definition
    part: &'a str, // part holds a reference, so the struct definition needs a lifetime annotation
}

fn first_word(s: &str) -> &str { // No annotations needed,bacause this definition matches one of the "lifetime elision rules"
    let bytes = s.as_bytes();    // I.e. the lifetime can be inferred.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}