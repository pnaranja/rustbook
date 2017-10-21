fn main() {
    /// VECTORS
    // Vectors are homogeneous
    let v : Vec<i32> = Vec::new();

    // Use vec! macro to create vector with initial values
    let v2 = vec![1,2,3];

    // How to push values in a Vector
    let mut v3 : Vec<i32> = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);

    // Reading elems of a vector
    let third_1: &i32 = &v2[2];
    let third_2: Option<&i32> = v2.get(2);

    let x = v2[2];
    println!("x: {}", x);
    println!("third_* is: {}", third_2.unwrap_or(&0));

    /// Remember, can't mutate when there is a mutable and immutable reference
    /// Pushing new value might force the compiler to reallocate new memory and
    /// copy old elements to the new space.  First element ref would be dangling.
//    let immutable_ref = &v3;
//    v3.push(0);


    // If you want different types you can use a vector of enums
    enum SpreadSheetCell{
        Int(i32), Float(f64), Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue"))
    ];

    /// STRINGS
    // Only 1 type of String in the Rust core - str (string slice)
    // Remember, str are a reference to some UTF-8 encoded string data stored elsewhere

    // String is in the Rust Std Library
    // It is a growable, mutable, owned UTF-8 string type
    let mut s : String  = String::new();
    let s : String = "A new String".to_string();
    let s : String = String::from("Another String");

    let mut s : String  = String::new();
    s.push_str("Try this");
    let mut s2 = " Ok!".to_string();
    s.push_str(&s2);
    s2 = s2.to_string();
    s2.push_str(" Again");

    println!("s: {}",s);
    println!("s2: {}",s2);
    println!("s1+s2={}",s + &s2);

    // '+' looks like an inline function of add
    // Definition: add(self, s: &str) -> String
    // s is now moved to the '+' function. Main function lost ownership of s
//    s.push_str("Uh oh");

    let s = String::from("Another!");
    // For >2 strings to concatenate, use format.  Returns a String
    println!("Using format!: {}", format!("Attaching strings: {} - {}", s, s2));

    //Strings are actually of type Vec<u8> (unsigned 8 bit or unsigned 1 byte))
    println!("Length of Hello is {}", "Hello".to_string().len()); // 5 bytes - each letter is a byte
    println!("Length of дра is {}", "дра".to_string().len()); // 6 bytes! - UTF-8 requires 2 bytes each

    // Therefore an index to a string's bytes will not always correlate to a valid Unicode scalar value
    // b should get 104 (byte value), but that's not really expected!
    // To avoid confusion, the Compiler will not allow this
    //let b = &"hello"[0];


    // Use chars method to access elements in a String
    [1,2,3,4].iter().for_each(|x| println!("{}",x));
    "sdf".chars().for_each(|x| println!("{}",x));
    let some_char : String = "Здравствуйте".chars().filter(|x| x.eq(&'й')).collect();
    println!("some char of Здравствуйте: {}", some_char);


    // Use String splice to get the first character (first byte in this case)
    let sdf = &"sdf"[0..1];
    println!("First letter of sdf is {}", sdf); // First letter of sdf is s

}

