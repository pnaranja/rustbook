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


}
