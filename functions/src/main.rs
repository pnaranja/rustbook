fn main() {
    println!("Hello, world!");
    println! ("Using plus_two: {} " , plus_two (3));

    let s = {
        45
    };

    println! ("s is {}", s);

    println! ("Check1: {}", check (true));
    println! ("Check2: {}", check2 (true));

    print_array ([2,3,4]);
    print_array2 ([6,7,8]);

    println! ("Fahrenheit to Celsius: {}", f_to_c(80.0));
    println! ("Celsius Fahrenheit: {}", c_to_f(30.0));

    println!("\nUsing higher order function");
    println! ("4 + 2 = {}",high_func(4, plus_two));

    println!("\nUsing anonymous function as a parameter");
    println! ("4 * 2 = {}", high_func(4, |x| x*2));

}



fn plus_two (x : i32) -> i32
{
    x + 2
}

fn check (x : bool) -> i32
{
    let num = if x { 3 } else {4};
    num
}

fn check2 (x : bool) -> i32
{
    if x { 3 } else {4}
}

fn print_array (x : [i32;3])
{
    for elem in x.iter ()
    {
        println! ("x array: {}", elem)
    }
}

fn print_array2 (x : [i32;3])
{
    let n = x.iter().map(|y| y + 2).collect::<Vec<i32>>();
    for elem in n.iter(){println!("z array: {}", elem)}
}

fn f_to_c(x: f32) -> f32
{
     (x - 32.0) * (5.0/9.0)
}

fn c_to_f(x: f32) -> f32
{
    x * (9.0/5.0) + 32.0
}

fn high_func(val: i32, f:fn(i32) -> i32) -> i32
{
    f(val)
}
