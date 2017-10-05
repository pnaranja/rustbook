fn main() {
    // Will not compile because s1 was moved to s2
//    let s1 = String::from("Paul");
//    let s2 = s1;
//    println!("{}", s1)

    // Use clone to make a deep copy - copy the heap data
    let s1 = String::from("Paul");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    let x = "hi";
    let y = x;

    let x2 = 10;
    let y2 = x2;

    println!("x: {}, y: {}", x, y);
    println!("x2: {}, y2: {}", x2, y2);

    ownership_test()
}

fn ownership_test()
{
    let s = String::from("hello");
    let s2 = s + " paul";
    take_ownership(s2);

    // This will do a compile error
    // let s3 = s + + s2 + "asf";

    let y = 45;
    function_copy(y)
}

fn take_ownership(s : String)
{
    println!("I have the string: {}", s)
}

fn function_copy(i : i32)
{
    let a = i + 3;
    println!("Copied the number {} and added 3 to get {}", i, a)
}
