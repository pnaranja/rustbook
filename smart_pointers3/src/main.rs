use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    //      |-------------------------------------------------------------------
    //      |                                                                  |
    //      v                                                                  |
    Cons(i32, RefCell<Rc<List>>),
    //RefCell so we can modify the List, that Cons is referring to
    Nil,
}

impl List {
    /// So we can access the 2nd item if we have a Cons variant
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item), //Using ref item so item is not moved?
            Nil => None
        }
    }
}


/// Create two lists that point to each other
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial Rc count: {}", Rc::strong_count(&a));
    println!("a = {:?}", a);
    println!("a.tail() = {:?}", a.tail().unwrap());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("\na Rc count after b creation: {}", Rc::strong_count(&a));
    println!("b initial Rc count: {}", Rc::strong_count(&b));
    println!("b = {:?}", b);
    println!("b.tail() = {:?}", b.tail().unwrap());

    // Change a's Nil to point to b
    // a.tail() saves a reference of the RefCell<Rc<Nil>> into link
    // Then change link to b
    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Both Rc counts will be 2
    println!("\na Rc count after changing a: {}", Rc::strong_count(&a));
    println!("b Rc count after changing a: {}", Rc::strong_count(&b));

    // Uncommenting this should overflow stack because we have a reference cycle
    //println!("a = {:?}", a);
}
