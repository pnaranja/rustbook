enum IpAddrKind{
    V4, V6
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
}

enum IpAddr2{
    V4(String),
    V6(String)
}

// Describe the format of V4 and V6
enum IpAddr3{
    V4(i8, i8, i8, i8),
    V6(String)
}


enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

// Define methods on enums
impl Message{
    fn call(&self){
        println!("Enum call")
    }
}

fn main() {
    let ip4 = IpAddrKind::V4;
    let ip6 = IpAddrKind::V6;

    let home = IpAddr{kind: IpAddrKind::V4, address: String::from("127.0.0.1")};
    let loopback = IpAddr{kind : IpAddrKind::V6, address: String::from("::1")};

    // Use an enum to describe home and loopback
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127,0,0,1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();

    // Using Optional Enum
    let b :Option<i32> = None; // Need to explicitly show what type of Optional for Empty
    let a = Some(3);
    let a2 = Some(6);
    let a_a2 = mul(a,a2);
    let a_b = mul(a,b);

    println!("a * a2 = {:?}",a_a2);
    println!("a * b = {:?}",a_b)
}

fn mul(a : Option<i32>, b: Option<i32>) -> i32
{
    // and_then is like flat map
    a.and_then(|x| b.and_then(|y| Some(x*y))).unwrap_or(0)
}
