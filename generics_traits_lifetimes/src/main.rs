fn largest_i32(list : &[i32]) -> i32
{
    list.iter().fold(0, |acc,&b| if acc>b {acc.clone()} else {b.clone()})
}

fn largest_char(list : &[char]) -> char
{
    list.iter().fold('a', |acc,&b| if acc>b {acc.clone()} else {b.clone()})
}

fn main() {
    let num_list = vec![1,4,5,3,9,3,4,6,8];
    let char_list = vec!['a','b','z','d','y','p'];

    println!("Largest number is {}", largest_i32(&num_list));
    println!("Largest char is {}", largest_char(&char_list));
}
