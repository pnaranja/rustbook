struct User{
    username : String,
    email : String,
    sign_in_count: u64,
    active: bool
}

// Tuple structs
// Structs with types but no names
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    let user1 = User{
        email : String::from("paul.naranja@gmail.com"),
        username : String::from("pnaranja"),
        active: true,
        sign_in_count : 1
    };
    println!("user1 username is {}", user1.username);

    // Create instance from other instances
    // use .. to specify that remaining fields have the same values
    let user2 = User{
        email : String::from("paul.naranja2@gmail.com"),
        username : String::from("pnaranja2"),
        ..user1
    };
    println!("user2 sign in count is {}", user2.sign_in_count);

    // Tuple structs
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // Calculate area of rectangle
    // Passing reference so main can still use rect
    let rect = Rectangle {length: 50, width: 30};
    let area = rect_area2(&rect);
    println!("Area of rectangle is {}", area);
    // Need :? to println the Rectangle Struct
    println!("For rectangle {:?}", rect);


    // Using Rectangle2 that has an area method to associated to it's struct
    let rect2 = Rectangle2{
        length : 100,
        width : 200
    };

    let area2 = rect2.area();
    println!("Rectangle2 area is {}", area2);

    let rect3 = Rectangle2{length:50, width:100};
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    let square = Rectangle2::square(50);
    println!("Size of the square is {:?}", square);


    // Rust 1.20 now has associated consts
    println!("Rectangle2 constant: {}", Rectangle2::rect_const);

}

// Field init shorthand
fn build_user(email:String, username:String) -> User
{
    User {
        email,
        username,
        active:true,
        sign_in_count:1
    }
}

// Calculate area of rectangle using tuple
// But not clear which is width and length?
fn rect_area(dimensions : (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

// Compiler says derive(Debug) is needed to println the Struct
#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32
}

fn rect_area2(dimensions : &Rectangle) -> u32{
    dimensions.length * dimensions.width
}

// Now define another Rectangle with the method area
#[derive(Debug)]
struct Rectangle2{
    length: u32,
    width: u32
}

impl Rectangle2{
    // Parameter is a reference to itself since the method is tied to the instance of the Rectangle2 struct
    fn area(&self) -> u32{
        self.length * self.width
    }

    fn can_hold(&self, other_rect : &Rectangle2) -> bool{
        (self.width > other_rect.width) && (self.length > other_rect.length)
    }

    // Associated function
    // Functions that do not have self as parameter
    // Like a Java static method
    fn square(size : u32) -> Rectangle2{
        Rectangle2{length: size, width: size}
    }

    // Rust 1.20 now has associated consts
    const rect_const : u32 = 50;
}
