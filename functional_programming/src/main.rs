extern crate rand;

use std::thread;
use std::time::Duration;
use rand::Rng;

struct Memoizer<T> where T: Fn(i32) -> i32{
    calculation : T,
    value: Option<i32>
}

impl<T> Memoizer<T> where T:Fn(i32) -> i32{
    fn new(calc_func : T) -> Memoizer<T>{
        Memoizer{calculation: calc_func, value: None}
    }

    pub fn get_value(&mut self, intensity: i32) -> i32{
        match self.value{
            Some(_v) => intensity,
            None => {
                let ret = (self.calculation)(intensity);
                self.value = Some(ret);
                ret
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    let random_wait_secs = rand::thread_rng().gen_range (3,6); //random number between 3-5

    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(random_wait_secs));
    intensity
}

// Refactor function to only need to run expensive calculation once
fn generate_workout<T:Fn(i32) -> i32>(calculation: T, intensity: i32){
    let mut memoized_func = Memoizer::new(calculation);

    if intensity < 25{
        println!("Do {} pushups!", memoized_func.get_value(intensity));

        println!("Do {} situps!", memoized_func.get_value(intensity));
    }
    else {
        // The expensive calculation should not be called at all, in this if block
        if intensity%2==0{
            println!("Take a break today!");
        }
        else{
            println!("Run for {} minutes", memoized_func.get_value(intensity));
        }
    }
}

fn main(){
    generate_workout(simulated_expensive_calculation, 23);

}
