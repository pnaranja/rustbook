use std::fs::File;
use std::io::Read;
use std::process;
use std::error::Error;

fn exit_gracefully<E: std::fmt::Debug, T: std::fmt::Debug>(msg: E) -> T{
    println!("Problem parsing arguments: {:?}", msg);
    process::exit(1);
}

#[derive(Debug)]
pub struct Config{
    query: String,
    filename : String
}

impl Config{

    /// Creates a Config struct of the parameters
    pub fn new(args: &[String]) -> Config {
        Config::parse_args(args).unwrap_or_else(exit_gracefully)
    }

    fn parse_args (args: &[String]) -> Result<Config, &'static str>{
        if args.len() != 3 {return Err("USAGE: minigrep <query> <filename>");}
        Ok (Config{query : args[1].clone() , filename : args[2].clone() })
    }
}

/// Retrieve the contents of the file in the config
pub fn get_contents(config: Config) -> String {
    read_from_contents(config)
        .unwrap_or_else(|err| {
            println!("Application error: {}", err);
            process::exit(1);
        })
}

fn read_from_contents(config: Config) -> Result<String, Box<Error>> {
    let mut f = open_file(config);
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn open_file (config: Config) -> File{
    File::open(config.filename).unwrap_or_else(exit_gracefully)
}
