use std::ops::Deref;
use std::fmt::Debug;

enum List{
    Cons (i32, Box<List>),
    Nil
}

use List::{Cons,Nil};

/// Box<T> puts value on the heap instead of the stack<br>
/// Box<T> is a pointer to a value on the heap<br>
///<br>
/// Rust cannot determine the size up front for recusive values<br>
/// But Rust can determine the size of the Box pointer<br>
///<br>
/// Box<T> is a "smart pointer" because it implements the Deref and Drop traits 
/// which allows Box<T> to be treated like a reference<br>
fn boxes() {
    let _list = Cons (1, Box::new(Nil));
    let _list2 = Cons (1, Box::new(Cons (2, Box::new (Nil))));

    let x = 5;
    let y = Box::new(x) ;

    assert_eq! (5, x);
    assert_eq! (5, *y); // *(y.deref())
}


/// myBox is defined in shorthand to:
/// ```
/// struct myBox<T>{
///     x : T
/// }
/// ```
struct myBox<T: Debug> (T);

impl<T: Debug> myBox<T>{
    fn new (x:T) -> myBox<T>{
        myBox(x)
    }
}

impl<T: Debug> Deref for myBox<T>{
    type Target = T; // Target is an associated type

    fn deref (&self) -> &T{
        &self.0 // access a struct's first field with self.0
    }
}

fn use_my_box (){
    let x = 5;
    let y = myBox::new(x) ;

    assert_eq! (5, x);
    assert_eq! (5, *y); // myBox needs to implement DeRef

    std::mem::drop(y);
    println!("Dropping myBox earlier");
}

/// Rust can call deref multiple times in order to get the correct type
fn implicit_deref_coercion(){
    let name = myBox::new(String::from("Paul"));
    
    // Rust turns &myBox<String> -> &String by calling deref.  Then &String -> &str by calling deref again
    print_inside_box(&name); 

}

fn print_inside_box(a : &str){
    println! ("Oh, hello {}", a);
}

impl<T : Debug> Drop for myBox<T>{
    fn drop(&mut self){
        println!("Dropping myBox with data: {:?}", self.0);
    }
}


//Using Rc for the List
use std::rc::Rc;
use RcList::{RcCons, RcNil};

enum RcList{
    RcCons(i32, Rc<RcList>),
    RcNil
}


/// Trying to create two lists that share ownership of a third list
fn three_lists(){
    let _a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let _b = Cons(3, Box::new(_a));

    // Won't work because a was already moved to b
//    let _c = Cons(4, Box::new(_a));
}

/// Using Rc allows a single value ("_a") to have multiple owners (via immutable refs)
/// Using Rc::clone increasing the reference count to a
/// We could have used a2.clone() but that would have been a deep copy
fn three_lists_using_rc(){

    let __a = RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil))));
    let _a = Rc::new(__a); //need to create a new instance of Rc of the RcList in order for Rc::clone
    println!("\nRef count after creating _a: {}", Rc::strong_count(&_a));

    let _b = RcCons(3, Rc::clone(&_a));
    println!("Ref count after creating _b: {}", Rc::strong_count(&_a));

    {
        let _c = RcCons(4, Rc::clone(&_a));
        println!("Ref count after creating _c: {}", Rc::strong_count(&_a));

    }
    println!("Ref count after _c is out of scope: {}", Rc::strong_count(&_a));

}

use std::cell::RefCell;

/// RefCell<T> will have Rust check borrowing rules at RUNTIME instead of compile time
fn mut_borrow_to_immut_value(){
    let x = vec![3,4];
    //let y = &mut x; // won't work because x is not mutable
    let y = RefCell::new(x);
    y.borrow_mut().push(5);

    assert_eq! (y.into_inner(), vec![3,4,5]);

}

fn main (){
    boxes();
    use_my_box();
    implicit_deref_coercion();
    three_lists();
    three_lists_using_rc();
    mut_borrow_to_immut_value();
}
