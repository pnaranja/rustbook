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

    ownership_test();

    // calculate length is borrowing the s1 variable?
    let s1_length = calculate_length(&s1);
    println!("Length of String {} is {}", s1, s1_length);

    let mut m1 = String::from("Paul");
    let m1_length = calculate_length_mutate(&mut m1);
    println!("Length of String {} is {}", m1, m1_length);

    // Can't mix mut and immutable refs
    let m2 = &m1;
    let m3 = &m1;
    //let m4 = &mut m1;
    println!("{} is {}", m2, m3);

    // No dangling refs
    let good_ref = get_ref();
    println!("{}",good_ref)

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

fn calculate_length(s: &String) -> usize
{
//    Cannot mutate a reference
//    s.push_str(" and hello");
    s.len()
}

fn calculate_length_mutate(s : &mut String) -> usize
{
//    Can mutate a mutable reference
    s.push_str(" Orange");
    s.len()
}

fn get_ref() -> String
{
    let s = String::from("Bye!");
    s
}

