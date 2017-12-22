extern crate minigrep;

use minigrep::Config;
use minigrep::get_contents;
use minigrep::search;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let query = &config.query;

    search(&query, get_contents(&config).as_str()).iter()
        .for_each(|found|println!("{:?}", found));
}

