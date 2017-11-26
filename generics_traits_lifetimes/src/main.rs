extern crate generics_traits_lifetimes;

use std::cmp::Ord;
use generics_traits_lifetimes::Tweet;
use generics_traits_lifetimes::Summarizable;


fn largest_i32(list : &[i32]) -> i32
{
    list.iter().fold(0, |acc,&b| if acc>b {acc.clone()} else {b.clone()})
}

fn largest_char(list : &[char]) -> char
{
    list.iter().fold('a', |acc,&b| if acc>b {acc.clone()} else {b.clone()})
}

fn largest<T : Clone + Ord >(list : &[T]) -> T
{
    let mut lst = list.to_owned();
    lst.sort();
    lst.last().unwrap().clone()
}




fn main() {
    let num_list = vec![1,4,5,3,9,3,4,6,8];
    let char_list = vec!['a','b','z','d','y','p'];

    println!("Largest number is {}", largest_i32(&num_list));
    println!("Largest char is {}", largest_char(&char_list));

    println!("Using Generic: Largest number is {}", largest(&num_list));
    println!("Using Generic: Largest char is {}\n\n", largest(&char_list));

    let tweet1 = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("New tweet: {}", tweet1.summary());
}
