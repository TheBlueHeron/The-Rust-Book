struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // Adding the annotation to derive the Debug trait and printing the Rectangle instance using debug formatting
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User { // the entire instance must be mutable to be able to change any field
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("anotherusername123");
    
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // this is equivalent to the previous assignment
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let _user = build_user(String::from("xyz@example.com"), String::from("builtinuser")); // no field is mutable after assignment

    struct Color(i32, i32, i32); // 'Tuple structs'
    struct Point(i32, i32, i32); // Not interchangeable with Color despite same number of fields with identical types

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let _area = area(&rect1); // clear and concise (no need to pass multiple values)

    println!( // {:?} is debug format for the println! macro
              // {:#?} is pretty-print debug format
        "The area of {:?} is {} square pixels.",
        rect1,
        rect1.area() // equivalent to area(&rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle { // smaller than rect1
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle { // bigger than rect1
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let _square = Rectangle::square(32); // associated function defined in the 'impl Rectangle' block below
}

// Using the Field Init Shorthand when Variables and Fields Have the Same Name
fn build_user(email: String, username: String) -> User {
    User {
        email, // Because the email field and the email parameter have the same name,
               // we only need to write email rather than email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 { // using a reference to the Rectangle struct this function is now clearly defined
    rectangle.width * rectangle.height
}

impl Rectangle { // implementing the area function as a method on the struct itself is even more clear
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool { // take an immutable borrow of another Rectangle as a parameter
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle { // 'associated function' that creates and returns a square
        Rectangle {                     // it is associated with the type but doesn't need an instance of the type
            width: size,
            height: size,
        }
    }
}