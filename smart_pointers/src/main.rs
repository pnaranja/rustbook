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

fn main (){
    boxes();
    use_my_box();
    implicit_deref_coercion();
}
