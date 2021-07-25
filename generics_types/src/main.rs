use std::cmp;
use std::marker;

fn main() {
    largest_number_duplicated();
    largest_number_extracted();
    largest_item_duplicated();
    largest_item_generic();
}

fn largest_number_duplicated() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = i32::MIN; // number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = i32::MIN; // number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
fn largest_number_extracted() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
}
fn largest_item_duplicated() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
fn largest_item_generic() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list { // see: 'Patterns and Matching'
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd + std::marker::Copy>(list: &[T]) -> T { // use plus sign to define multiple constraints, called 'trait bounds'
    let mut largest = list[0]; // type needed that can be copied -> add std::marker::Copy constraint

    for &item in list {
        if item > largest { // type needed that can be ordered and thus compared -> add std::cmp::PartialOrd constraint
            largest = item;
        }
    }
    largest
}

fn largestReference<T: PartialOrd>(list: &[T]) -> &T { // returning a reference removes the need for copy trait and heap allocations
    let mut largest = &list[0]; // no longer type needed that can be copied

    for item in list {
        if item > largest { // type needed that can be ordered and thus compared -> add std::cmp::PartialOrd constraint
            largest = &item;
        }
    }
    largest
}