extern crate minigrep;

use minigrep::Config;
use minigrep::get_contents;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Here is the contents of the file '{}':\n\n{}",args[2], get_contents(config));
}
