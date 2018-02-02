use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    //                     |-----------------------------------------
    //                     |                                        |
    //                     v                                        |
    Cons(i32, RefCell<Rc<List>>),
    //Use RefCell so we can modify the List, that Cons is referring to
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
fn cyclical_references() {
    //Rc::strong_count increases the strong count of the Rc instance
    //Rc instances will be cleaned only after the strong count is 0

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
    println!("\n\n\n")
}


// Create weak references by Rc::downgrade
// Weak count does NOT need to be 0 for the Rc instance to be cleaned up
// Therefore need to check if weak reference is still valid by calling upgrade which returns
// an Option<Rc<T>>

// Node that has it's value and reference to it's childrens' nodes (Vec<Rc<Node>>)
// Rc<Node>: Node should be able to share children ownership with variables to access each Node directly
// RefCell<T>: To modify which nodes are children of another node
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}


fn tree_example() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    // branch can access leaf by branch.children
    let branch = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // But no way for leaf to access branch!
    // Node should add a parent reference
}

// Dropping a leaf DOES NOT mean you have to drop the branch -> should make this a weak reference
#[derive(Debug)]
struct Node2 {
    value: i32,
    children: RefCell<Vec<Rc<Node2>>>,
    parent: RefCell<Weak<Node2>>,
}

fn tree_example2() {
    //Create and then mutate it's parent!
    let leaf = Rc::new(Node2 {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    let branch = Rc::new(Node2 {
        value: 3,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // mutating the RefCell's weak reference
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // upgrade() returns an Optional Rc
    println!("Parent's leaf is {:?}", leaf.parent.borrow().upgrade().unwrap());
}

fn main() {
    cyclical_references();
    tree_example2()
}
