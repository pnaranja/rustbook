use std::env;
use std::fs::File;
use std::io::Read;
use std::io::prelude;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
                    .unwrap_or_else(|err| {
                        println!("Problem parsing arguments: {}", err);
                        process::exit(1);
                    });

    print_contents(config);
}

struct Config{
    query: String,
    filename : String
}

impl Config{
    fn new (args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {return Err ("Not enough arguments");}
        Ok (Config{query : args[1].clone() , filename : args[2].clone() })
    }
}

fn print_contents (config: Config){
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read the file");

    println!("{}",contents);
}
