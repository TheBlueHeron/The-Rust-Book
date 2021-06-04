mod front_of_house; // Using a semicolon tells Rust to load the contents of the module from another file
                    // with the same name as the module

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast { // defined here, else no access to private seasonal_fruit field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer { // all variants of this enum are also public
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting; // or: use self::front_of_house::hosting; // this scope is private!
// pub use crate::front_of_house::hosting; // re-exporting a name by scoping it as public;
use crate::front_of_house::hosting::add_to_waitlist; // unidiomatic, i.e. code is unclear as to where add_to_waitlist is defined
use std::collections::HashMap; // idiomatic to specify the full path when referring to structs and enums

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist(); // valid as a result of the 'use' statement
    add_to_waitlist(); // valid as a result of the 'use' statement
    
    let mut map = HashMap::new();  // valid as a result of the 'use' statement
    map.insert(1, 2);
}

use std::fmt::Result;
use std::io::Result as IoResult; // prevent bringing types with the same name into the same scope using 'as' keyword

fn function1() -> Result {
    return Result::Ok(())
}

fn function2() -> IoResult<()> {
    return IoResult::Ok(())
}

// Nested use paths:

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write}; // This line brings std::io and std::io::Write into scope

// The Glob operator
use std::collections::*; // bring all public items defined in a path into scope