/// Rust defaults to look first into lib.rs

pub mod client;

pub mod network;

// Use 'use' so no need to specify in between modules
use outermost::inside;
use outermost::inside::inner_function;

// You can also use 'use' for enums
enum TrafficLight{Red,Yellow,Green}
use TrafficLight::{Red,Green};

enum TrafficLight2{Red2,Yellow2,Green2}
use TrafficLight2::*;



mod outermost{
    pub fn middle_function(){}
    pub fn middle_secret_function(){}

    pub mod inside{
        pub fn inner_function(){}
        pub fn secret_function(){}
    }
}


fn try_me(){
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();

    // Using 'use' access inner functions
    inside::secret_function();
    inner_function();

    // Using 'use' access enum values
    let red = Red;
    let yellow = TrafficLight::Yellow;
    let green = Green;

    let red2 = Red2;
    let yellow2 = Yellow2;
    let green2 = Green2;
}




#[cfg(test)]
mod tests {

    use super::client;
    #[test]
    fn it_works() {
        // use 'super' (or implied nothing) to go up the 'tests' module and access sibling modules
        super::client::connect();
        ::client::connect();
    }

    // using the higher level 'use super::client'
    fn it_works2() {
        client::connect();
    }
}
