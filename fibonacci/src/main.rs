fn main() {
    println!("Fib number 3: {}", fib(3));
    println!("Fib number 10: {}", fib(10));
    println!("Fib number 20: {}", fib(20));

    // Panic when run in debug mode.  Able to finish in release mode
//    println!("Fib number 50: {}", fib(50));
}

fn fib(x : u32) -> u32
{
    match x
    {
        0 => 0,
        1 => 1,
        _ => fib(x-2)+fib(x-1)
    }
}
