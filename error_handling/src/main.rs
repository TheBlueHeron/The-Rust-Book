use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // see also: Cargo.toml for the panic switch on what to do when panicking
    // Backtrace:
    // Ensure that environment variable 'RUST_BACKTRACE' is present in system environmnent variables
    // bash: export RUST_BACKTRACE=1 or "full"
    // cmd: > set RUST_BACKTRACE=1 or "full"
    // powershell: > $Env:RUST_BACKTRACE=1 or "full"
    // then use: > cargo run
    // e.g. panic!("crash and burn");

    open_file_1(); // simple success or panic
    open_file_2(); // advanced matching
    open_file_3(); // same, but using closures (see iterators_and_closures)
    let _r: Result<String, io::Error> = open_file_4(); // simple propagation
    let _r: Result<String, io::Error> = open_file_5(); // propagation using the ? operator
    let _r: Result<String, io::Error> = open_file_6(); // the above code is built-in (in std::fs)
}

fn open_file_1() {
    let f = File::open("hello.txt"); // returns a Result<File, io::Error>

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // same as: let f = File::open("hello.txt").unwrap(); or ... .expect("Friendly message");
}

fn open_file_2() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn open_file_3() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn open_file_4() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn open_file_5() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // even better when chaining method calls:
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}

fn open_file_6() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // return result immediately
}