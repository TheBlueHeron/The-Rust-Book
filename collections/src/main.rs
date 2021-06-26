fn main() {
    vectors();
    strings();
    hashmaps();
    exercises();
}

fn vectors() {
    // construction
    let mut v: Vec<i32> = Vec::new(); // Vectors are implemented using generics
    let mut v2 = vec![1, 2, 3]; // all items must be of the same type (infered as i32 here by the Vec! macro)

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Access
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(4) { // get returns an Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let first = &v2[0];

    // v2.push(6); // don't mix mutable and immutable borrows

    println!("The first element is: {}", first);
    v2.push(5); //  no problem, as there is no more immutable reference after this line

    // Iteration
    for i in &v { // immutable reference
        println!("{}", i);
    }
    for i in &mut v { // mutable reference
        *i += 50; // change dereferenced value
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn strings() {
    // construction
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string(); // string literals implement the 'Display' trait
    let s = "initial contents".to_string(); // the method also works on a literal directly
    let s = String::from("initial contents"); // equivalent to previous line

    let hello = String::from("السلام عليكم"); // String are UTF-8 encoded
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते"); // len = 18, not 4 or 6!
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте"); // len = 24, not 12!
    let hello = String::from("Hola"); // len = 4

    // Updating
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str takes a reference to a string slice, so the next line is valid
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l'); // push takes a single character

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // because the + operator uses the add method:
                       // fn add(self, s: &str) -> String {... not exact definition but the first param is owned, the second is borrowed
                       // also: &s2 is coerced into &s2[..], which is a slice (i.e. by deref coercion)

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // efficient concatenation of multiple strings by using format! macro

    // Indexing
    let s1 = String::from("hello");
    // let h = s1[0]; // this is NOT valid, because String is a wrapper over a Vec<u8>

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // use with caution, because &hello[0..1] will panic

    for c in "नमस्ते".chars() { // iterate over the result of the chars() method instead
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() { // or iterate over the result of the bytes() method
        println!("{}", b);
    }

}

use std::collections::HashMap;

fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // all keys are of the same type and all values are of the same type as well
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // returns Option<i32>, in this case Some(&10)

    for (key, value) in &scores { // iterate over all key-value pairs
        println!("{}: {}", key, value);
    }

    let mut teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = // infer the types that the hash map contains based on the types of the data in the vectors by using <_, _>
    teams.into_iter().zip(initial_scores.into_iter()).collect(); // create vector of tuples and collect them into the HashMap
    // teams.clear(); // Cannot do this; HashMap has taken ownership

    // Updating
    scores.insert(String::from("Blue"), 25); // simple replace!
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // entry method returns an Entry enum. or_insert always returns a mutable reference to the appropriate value
    scores.entry(String::from("Yellow")).or_insert(50); // insert only when key has no value
    scores.entry(String::from("Blue")).or_insert(50); // insert only when key has no value

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // count is a mutable reference
        *count += 1; // dereference and update value
    }

    println!("{:?}", map);
}

fn exercises() {
    
}