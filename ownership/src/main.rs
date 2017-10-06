fn main() {
    // Will not compile because s1 was moved to s2
//    let s1 = String::from("Paul");
//    let s2 = s1;
//    println!("{}", s1)

    // Use clone to make a deep copy - copy the heap data
    let s1 = String::from("Paul");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    let x : &str = "hi";
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
    println!("{}",good_ref);

    let p : &str = "asfs";
    let d = (1,2);

    // Slices
    // Return the first word in a string
    // Range using <a>..<b> where inclusive a -> exclusive b
    let mut f = String::from("Hello World");
    let hello = &f[0..5]; println!("{}", hello);
    let world = &f[6..11]; println!("{}", world);
    let hello2 = &f[..5]; println!("{}", hello2);
    let world2 = &f[6..]; println!("{}", world2);
    let hello_world = &f[..]; println!("{}", hello_world);
    println!("First word of 'Hello World': {}",get_first_word(&f));
    let firstword = get_first_word(&f);

    // Cannot take a mutable ref since it's there's an immutable reference in the same scope
    // f.clear();

    // String literals are immutable references (slices)
    let g = String::from("Hello World");
    println!("{}", &g[..]);
    let h = "Hello World";
    println!("{}", h);

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

fn get_first_word(s : &String) -> &str
{
    // as_bytes returns an array of bytes (8 bits)
    let bytes : &[u8] = s.as_bytes();

    // enumerate will return a tuple (index, item_ref)
    for (i, &item) in bytes.iter().enumerate()
    {
        // look for a byte space
        if item == b' '
        {
            return &s[..i]
        }
    }
    &s[..]
}

fn get_first_word2(s : &str) -> &str
{
    // as_bytes returns an array of bytes (8 bits)
    let bytes : &[u8] = s.as_bytes();

    // enumerate will return a tuple (index, item_ref)
    for (i, &item) in bytes.iter().enumerate()
        {
            // look for a byte space
            if item == b' '
                {
                    return &s[..i]
                }
        }
    &s[..]
}
