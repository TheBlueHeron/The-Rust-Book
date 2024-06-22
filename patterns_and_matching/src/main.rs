
// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

fn main() {

    // places where patterns are used:
    if_let_expressions();
    while_let_loops();
    for_loops();
    let_statements();
    function_parameters();

    // refutability: whether a pattern might fail to match
    // let x =  5; irrefutable
    // if let Some(x) = a_value; refutable
    // let Some(x) = some_option_value; can't use a refutable pattern in a let statement
    // if let Some(x) = some_option_value { // refutability is ok here (and expected in fact)
    //     println!("{}", x);
    // }
    // if let x = 5 { // ok, but doesn't make sense -> compiler warning
    //    println!("{}", x);
    // };

    pattern_syntax();

}

fn if_let_expressions() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color { // new shadowed variable color inside Some variant
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // new shadowed variable age inside Ok variant
        if age > 30 { // can’t combine these two conditions into if let Ok(age) = age && age > 30.
                      // The shadowed age thaqt must be compared to 30 isn’t valid until the new scope starts with the curly bracket
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else { // all other contingencies; -> exhaustive, but can't be checked by compiler
        println!("Using blue as the background color");
    }

}

fn while_let_loops(){
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top); // code runs until stack.pop() returns None
    }
}

fn for_loops(){
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

}

fn let_statements(){

    let x = 5; // let PATTERN = EXPRESSION !
    let (x, y, z) = (1, 2, 3); // same same

}

fn function_parameters(){
    foo(1); // x is a pattern!
    let point = (3, 5); // &(3, 5) matches the pattern &(x, y), so x is the value 3 and y is the value 5
    print_coordinates(&point); // the tuple is a pattern
}

fn foo(x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn pattern_syntax(){

    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // last arm is irrefutable (i.e. matches anything)
    }

    // matching named variables
    
}