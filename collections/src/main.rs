fn main() {
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

}

