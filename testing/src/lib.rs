#[cfg(test)]
mod tests {

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
        assert!(!smaller.can_hold(&smaller));
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


