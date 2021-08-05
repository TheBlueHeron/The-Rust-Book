use adder;
mod common; // Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output
            // tests/common/mod.rs is accessible from any of the integration test files as a module

#[test]
fn adds_two() {
    common::setup(); // do some shared initialization
    assert_eq!(4, adder::add_two(2));
}

// for config examples, see:
// https://joshleeb.com/posts/rust-integration-tests.html

// see cargo.toml for integration config
// usage:
// # Run all tests
// cargo test

// # Run only unit tests
// cargo test --lib

// # Run only integration tests
// cargo test --test integration

// # Run only integration tests, single threaded
// # (you’ll probably want this one)
// cargo test --test integration -- --test-threads=1