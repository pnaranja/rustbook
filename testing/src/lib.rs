#[cfg(test)]
pub mod tests {


    // Set # of threads to use to run: cargo run -- --test-threads=1
    // Show debug output (like println statements): cargo run -- --nocapture
    // Run ignored tests: cargo test -- --ignored
    // Run a subset of tests: cargo test greeting

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    //#[test]
    //fn will_fail() {
    //    panic!("Failing test");
    //}

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};
        assert!(!smaller.can_hold(&larger));
    }


    pub fn greeting(name: &str) -> String{
        format!("Hello {}!", name)
    }

    #[test]
    fn greeting_test(){
        println!("THE GREETING TEST!!!!");
        assert_eq!(greeting("Paul"), "Hello Paul!");
    }

    // thread 'tests::greeting_test_error' panicked at 'Expected 'Hello Paul!' but received 'Hello Mike!''
    #[test]
    #[ignore]
    fn greeting_test_error(){
        assert!(greeting("Mike").eq("Hello Paul!"), "Expected 'Hello Paul!' but received '{}'", greeting("Mike"))
    }



    fn not_greater_than_100(value: i32) -> i32{
        if value > 100 {panic!("Cannot be greater than 100")}
        value
    }

    // Verify panics
    #[test]
    #[should_panic]
    fn greater_than_100(){
        not_greater_than_100(101);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn less_than_100(){
        not_greater_than_100(99);
    }

    #[test]
    #[ignore]
    #[should_panic(expected="Cannot be something")]
    fn wrong_panic_msg(){
        not_greater_than_100(102);
    }

    // Ignore this test
    #[test]
    #[ignore]
    fn ignore_this_test(){
        assert_eq!(1,1);
    }



    #[derive(Debug)]
    struct Rectangle{
        length: u32,
        width: u32
    }

    impl Rectangle{
        // Parameter is a reference to itself since the method is tied to the instance of the Rectangle2 struct
        fn area(&self) -> u32{
            self.length * self.width
        }

        fn can_hold(&self, other_rect : &Rectangle) -> bool{
            (self.width > other_rect.width) && (self.length > other_rect.length)
        }
    }

}

pub fn greeting2(name: &str) -> String{
    format!("Hello {}!", name)
}




