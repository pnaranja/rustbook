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
    assert_eq! (5, *y);
}

struct myBox<T> (T);

fn main (){
    boxes();
}
