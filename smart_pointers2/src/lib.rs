//! A library that tracks a current value compared to a max value<br>
//! The library sends status messages about the proximity between the current and max value
//!
//! The library does not care HOW the status messages are sent
//!

pub trait Messenger{
    fn send(&self, msg :&str);
}

pub struct LimitTracker<'a, T: 'a + Messenger>{
    messenger: &'a T,
    value : usize,
    max : usize
}

impl <'a, T : Messenger> LimitTracker<'a, T>{

    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
        LimitTracker{messenger, value : 0, max}
    }

    pub fn set_value_send_msg(&mut self, value: usize){
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9{
            self.messenger.send("Used up 75% of quota");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0{
            self.messenger.send("Used up 90% of quota");
        } else if percentage_of_max > 1.0{
            self.messenger.send("You are over your quota!");
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

/// New List that uses Rc and RefCell
/// It can have multiple owners and by mutable
#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}


/// In our tests, create an object to mock sending messages
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{sent_messages : RefCell::new(vec![])}
        }
    }

    /// We can't modify the sent_messages because the send method takes an
    /// immutable reference to self - which is defined in the Messenger trait
    /// So store the sent_messages within a RefCell!
    impl Messenger for MockMessenger{
        fn send(&self, msg : &str){
            self.sent_messages.borrow_mut().push(String::from(msg));

            // This will panic at runtime because of two mutable references
            // of the sent_messages vector in the same scope
//            let mut mut_borrow1 = self.sent_messages.borrow_mut();
//            let mut mut_borrow2 = self.sent_messages.borrow_mut();
        }
    }

    #[test]
    fn send_over_75_percent() {
        let mock_messenger = MockMessenger::new();

        let mut tracker = LimitTracker::new(&mock_messenger, 100);
        tracker.set_value_send_msg(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }


    // Test new List
    use List::{Cons, Nil};

    #[test]
    fn test_new_list(){
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Rc::new(Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)));
        let c = Rc::new(Cons(Rc::new(RefCell::new(9)), Rc::clone(&a)));

        *value.borrow_mut() += 20;

        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("c = {:?}", c);
        // a = Cons(RefCell { value: 25 }, Nil)
        // b = Cons(RefCell { value: 6 }, Cons(RefCell { value: 25 }, Nil))
        // c = Cons(RefCell { value: 9 }, Cons(RefCell { value: 25 }, Nil))
    }

}
