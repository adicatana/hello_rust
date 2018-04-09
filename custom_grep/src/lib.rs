use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }

    pub fn run(&self) -> Result<(), Box<Error>> {
        let mut f = File::open(&self.filename)?;

        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        println!("{:?}", contents);

        Ok(())

    }

}
