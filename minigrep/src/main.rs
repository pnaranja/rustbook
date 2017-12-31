extern crate minigrep_lib;

use minigrep_lib::Config;
use minigrep_lib::get_contents;
use minigrep_lib::search;
use minigrep_lib::search_case_insensitive;
use std::env;
use std::env::Args;

fn main() {
    let args : Args = env::args();

    let config = Config::new(args);
    let query = &config.query;

    if config.case_insensitive.eq("1") || config.case_insensitive.contains("true") {
        search_case_insensitive(&query, get_contents(&config).as_str())
            .iter().for_each(|found|println!("{:?}", found));
    }else{
        search(&query, get_contents(&config).as_str())
            .iter().for_each(|found|println!("{:?}", found));
    }
}

