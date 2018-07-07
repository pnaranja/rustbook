fn main() {
    // Matching Name Variables
    let x = Some (5);
    let y = 10;

    // Some(y) will match ANY value in Some and assign it to the local y in match{}
    match x{
        Some (50) => println! ("Should not match this"),
        Some (y) => println! ("Matched in inside scope y={}", y),
        _ => println! ("Should not match this")
    }

    println! ("After match x: x.unrapped={}, y={}", x.unwrap(), y);


    // Multiple Patterns
    match y{
        1 | 2 | 3 => println! ("Should not match this"),
        4 | 20 | 10 => println! ("Matched y using 4 | 20 | 10"),
        _ => println! ("Should not match this"),
    }


    // Match ranges
    match y{
        1...9 => println! ("Should not match this"),
        10...20 => println! ("Matched y using 10..20"),
        _ => println! ("Should not match this"),
    }

    // Destructuring Structs
    struct Point{
        x : i32, y: i32
    }

    let p = Point{x : 5, y: 9};
    let Point{x , y} = p;
    println! ("Point destructed: x = {}, y = {}", x, y);

    match p {
        Point {x , y: 0} => println!("On the x axis {}", x),
        Point {x : 0 , y} => println!("On the y axis {}", y),
        Point {x , y} => println!("On neither axis {} {}", x, y),
    }

    // Destructuring Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 106, 255);

    match msg {
        Message::Quit => println!("I Quit!"),
        Message::Move { x, y } => println!("Move direction x: {}, y: {}", x, y),
        Message::Write(x) => println!("Writing string: {}", x),
        Message::ChangeColor(x, y, z) => println!("Changing color rgb: {}, {}, {}", x, y, z)
    }

    // Destructuring References
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    // Rust 1.27 allows Point {x,y} to not need to explicitly show reference in the iter
    let sum_of_squares: i32 =
        points.iter().map(|Point { x, y }| x * x + y * y).sum();

    println!("Sum of squares of the points: {}", sum_of_squares);


    // Destructuring Structs and Tuples

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });
    println!("Feet: {}, Inches: {}, Points x: {}, y: {}", feet, inches, x, y);


    // Ignore Values in a pattern
    let something = Some(5);
    let something2 = Some(10);

    match (something, something2) {
        (Some(_), Some(_)) => println!("There was SOMETHING!"),
        _ => println!("There was NOTHING!")
    }

    struct Point2 { x: i32, y: i32, z: i32 }
    let origin = Point2 { x: 1, y: 2, z: 3 };

    match origin {
        Point2 { x, .. } => println!("Here's just x: {}", x),
        _ => println!("There's no Point!")
    }

    let nums = (1, 2, 3, 4, 5);
    match nums {
        (first, .., last) => println!("Here the first and last nums: {}, {}", first, last)
    }

    // Create reference patterns
    let robot_name = Some(String::from("Bors"));

    // Match the reference (&) instead, so robot_name can be printed after
    match &robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }

    let mut robot_name2 = Some(String::from("Cors"));
    println!("robot_name2 is: {:?}", robot_name2);

    match &mut robot_name2{
        Some(name) => *name = String::from ("Boris"),
        None => ()
    }
    println!("robot_name2 is now: {:?}", robot_name2);


    // Extra conditionals with Match Guards
    let num = Some (4);
    match num{
        Some (x) if x > 5 => println! ("x is greater than 5"),
        Some (x) => println! ("x is less than or equal to 5"),
        None => println! ("x is nothing!")
    }

    let num2 = Some (7);
    match num2{
        Some (3) | Some (4) |Some (5)  => println! ("x is 3 to 5"),
        Some (6) | Some (7) |Some (8)  => println! ("x is 6 to 8"),
        Some (_) => println! ("x is something"),
        None => println! ("x is nothing!")
    }

    // Bindings using @
    enum Message2{
        Hello {id:i32}
    }
    let msg = Message2::Hello{id : 5};
    match msg{
        Message2::Hello{id: id_var @ 1...4} => println! ("id is between 1-4: {}", id_var),
        Message2::Hello{id: id_var @ 5...7} => println! ("id is between 5-7: {}", id_var),
        _ => println! ("No idea what the id is")
    }
}
