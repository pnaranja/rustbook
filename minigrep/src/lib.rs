use std::env;
use std::fs::File;
use std::io::Read;
use std::io::prelude;
use std::process;
use std::error::Error;

pub struct Config{
    query: String,
    filename : String
}

impl Config{
    /// Creates a Config struct of the parameters
    pub fn new(args: &[String]) -> Config {
        Config::parse_args(args)
            .unwrap_or_else(|err| {
                println!("Problem parsing arguments: {}", err);
                process::exit(1);
            })
    }

    fn parse_args (args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {return Err ("Not enough arguments");}
        Ok (Config{query : args[1].clone() , filename : args[2].clone() })
    }
}

pub fn get_contents(config: Config) -> String {
    read_from_contents(config)
                    .unwrap_or_else(|err| {
                        println!("Application error: {}", err);
                        process::exit(1);
                    })
}

fn read_from_contents(config: Config) -> Result<String, Box<Error>> {
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

