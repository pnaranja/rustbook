extern crate rand;

use std::thread;
use std::time::Duration;
use rand::Rng;

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    let random_wait_secs = rand::thread_rng().gen_range (1,11);

    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(random_wait_secs));
    intensity
}

// Refactor function to only need to run expensive calculation once
fn generate_workout(intensity: i32){
    let expensive_closure = |intensity| {
        let random_wait_secs = rand::thread_rng().gen_range (1,11);

        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(random_wait_secs));
        intensity
    };


    if intensity < 25{
        println!("Do pushups!");
        simulated_expensive_calculation(intensity);
        println!("Do situps!");
        simulated_expensive_calculation(intensity);
    }
    else {
        // The expensive calculation should not be called at all if in this if block
        if intensity%2==0{
            println!("Take a break today!");
        }
        else{
            println!("Run for {} minutes", intensity);
            simulated_expensive_calculation(intensity);
        }
    }
}

fn main(){
    generate_workout(29);

}
