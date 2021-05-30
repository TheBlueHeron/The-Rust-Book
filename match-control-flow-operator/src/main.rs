enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // equivalent to 'Case Else' -> Do nothing
    }

    // In some case the 'if let' syntax is a good alternative
    let mut count = 0;
    let coin = Coin::Quarter(String::from("Montana"));

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// match, handling 'null' values
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,   // Matches in Rust are exhaustive: we must exhaust every last possibility
                        // in order for the code to be valid
        Some(i) => Some(i + 1),
    }
}