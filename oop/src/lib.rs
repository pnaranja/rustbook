// Library to draw "objects" to the "screen"

pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    // Create a vector of Draw "objects"
    // Draw can be implemented for any type so don't know the size at compile time -> Need Box
    // Box<Draw> is a trait object (rust compiler uses dynamic dispatch)
    pub components: Vec<Box<Draw>>
}

impl Screen {
    // Define a run method that "draws" all the components
    pub fn run(&self) {
        self.components.iter().for_each(|component| component.draw())
    }
}


// Add types to implement the Trait
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub length: i32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with width {}, height {}, and length {}"
                 , self.width
                 , self.height
                 , self.length);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
