use std::fs::File;
use std::io::Read;
use std::process;
use std::error::Error;
use std::env;

fn exit_gracefully<E: std::fmt::Debug, T: std::fmt::Debug>(msg: E) -> T{
    eprintln!("Problem running minigrep: {:?}", msg);
    process::exit(1);
}

#[derive(Debug)]
pub struct Config{
    pub query: String,
    filename : String,
    pub case_insensitive : String,
}

impl Config{

    /// Creates a Config struct of the parameters
    pub fn new(args: &[String]) -> Config {
        Config::parse_args(args).unwrap_or_else(exit_gracefully)
    }

    fn parse_args (args: &[String]) -> Result<Config, &'static str>{
        if args.len() != 3 {return Err("USAGE: minigrep <query> <filename>");}
        Ok (Config{query : args[1].clone() , filename : args[2].clone(),
            case_insensitive : env::var("CASE_INSENSITIVE").unwrap_or("false".to_string())})
    }
}

/// Retrieve the contents of the file in the config
pub fn get_contents(config: &Config) -> String {
    read_from_contents(config).unwrap_or_else(exit_gracefully)
}

fn read_from_contents(config: &Config) -> Result<String, Box<Error>> {
    let mut f = open_file(&config);
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn open_file (config: &Config) -> File{
    File::open(&config.filename).unwrap_or_else(exit_gracefully)
}


/// Search for the query in the contents
/// Lifetimes: Search results should live as long as the contents to search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|x| x.contains(query)).map(|x| x.trim()).collect()
}

/// Search for query (case insensitive)
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // to_lowercase creates new String.  However contains function requires string reference
    contents.lines().filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .map(|x| x.trim()).collect()
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents =
            "Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_results(){
        let query = "lo";
        let contents =
            "Hello,
            How are you?
            How low can you go?";

        assert_eq!(vec!["Hello,","How low can you go?"], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let contents =
            "Rust is great
            But is rust better than go?
            I don't know!";

        assert_eq!(vec!["Rust is great","But is rust better than go?"],
                   search_case_insensitive(query, contents));
    }
}