fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    enum IpAddr { // each variant has an associated value
        V4(u8, u8, u8, u8),
        V6(String),
    }

    struct IpAddress {
        kind: IpAddrKind,
        address: String,
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message { // define method on enum using impl; same as structs
        fn call(&self) {
            // method body would be defined here
        }
    }

    // enum Option<T> { // defined in standard library as shown here
    //     Some(T),
    //     None,
    // }

    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Much more concise
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    // A better way of handling null values
    let absent_number: Option<i32> = None;
}