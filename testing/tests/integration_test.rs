extern crate testing;
mod common;

// Unit tests need to PASS before running integration tests

// To run just this test file: cargo test --test integration_test

// Binary crates that have src/main.rs BUT DO NOT HAVE src/lib.rs cannot create
// integration tests in the tests directory


#[test]
fn test1(){
    common::setup();
    assert_eq!(testing::greeting2("Mae"), "Hello Mae!");
}

#[test]
fn test2(){
    assert_eq!(testing::greeting2("Mae"), "Hello Paul!");
}
