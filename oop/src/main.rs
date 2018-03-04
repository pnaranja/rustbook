extern crate oop;

use oop::{Draw, Button, Screen};

// Draw a SelectBox
struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox with width {}, height {} and options:", self.width, self.height);
        self.options.iter().for_each(|option| println!("{}", option));
    }
}

fn main() {
    let select_box = SelectBox { width: 2, height: 2, options: vec!["on".to_string(), "off".to_string()] };
    let button = Button { width: 3, height: 3, length: 3 };

    let screen = Screen { components: vec![Box::new(select_box), Box::new(button)] };

    screen.run();
}