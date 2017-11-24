use std::cmp::Ord;


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

struct Point<T,U>{
    x : T,
    y: U,
}

impl<T,U> Point<T,U>{
    fn x (&self) -> &T{
        &self.x
    }

    fn y (&self) -> &U{
        &self.y
    }

    fn mixup<V,W>(self, other_point: Point<V,W>) -> Point<T,W>{
        Point{x : self.x, y: other_point.y}
    }
}

fn main() {
    let num_list = vec![1,4,5,3,9,3,4,6,8];
    let char_list = vec!['a','b','z','d','y','p'];

    println!("Largest number is {}", largest_i32(&num_list));
    println!("Largest char is {}", largest_char(&char_list));

    println!("Using Generic: Largest number is {}", largest(&num_list));
    println!("Using Generic: Largest char is {}", largest(&char_list));

    let int_and_float = Point {x : 5, y : 1.0};
    println!("Point x is {}", int_and_float.x ());
    println!("Point y is {}", int_and_float.y ());

    let char_and_string = Point {x: 'w', y: "Hello"};
    let mixup = int_and_float.mixup(char_and_string);

    println!("Mixup! x: {}, y: {}", mixup.x(), mixup.y())

}
