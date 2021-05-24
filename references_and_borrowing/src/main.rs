fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");
    change(&s); // error

    let mut s = String::from("hello");
    changeOK(&mut s); // ok

    // if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does
    let reference_to_nothing = dangle();
}
// These ampersands are references, and they allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &String) {
    some_string.push_str(", world"); // Just as variables are immutable by default, so are references.
                                     // We’re not allowed to modify something we have a reference to.
}

fn changeOK(some_string: &mut String) {
    some_string.push_str(", world"); // Mutable references have one big restriction:
                                     // you can have only one mutable reference to a particular piece of data in a particular scope
}

fn CombiningBorrows() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // error, you can have only one mutable reference to a particular piece of data in a particular scope

    println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM, we also cannot have a mutable reference while we have an immutable one

    println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!