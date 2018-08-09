use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args).unwrap_or_else(|error|{
            println!("{}", error);
        proccess
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut file = File::open(config.filename).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file!");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

