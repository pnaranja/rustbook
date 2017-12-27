extern crate minigrep;

use minigrep::Config;
use minigrep::get_contents;
use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let query = &config.query;

    println!("Case Insensitive value: {:?}", config.case_insensitive);

    if config.case_insensitive.eq("1") || config.case_insensitive.contains("true") {
        search_case_insensitive(&query, get_contents(&config).as_str())
            .iter().for_each(|found|println!("{:?}", found));
    }else{
        search(&query, get_contents(&config).as_str())
            .iter().for_each(|found|println!("{:?}", found));
    }
}

