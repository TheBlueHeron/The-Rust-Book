fn main() {
    let mut number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // is equivalent to:
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // is equivalent to:
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
