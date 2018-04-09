extern crate custom_grep;

use std::env;
use std::process;

use custom_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {:}", err);
        process::exit(1);
    });

    println!("Searching for {:}", config.query);
    println!("In the file {:}", config.filename);

    if let Err(e) = config.run() {
        println!("Application error {:}", e);
        process::exit(1);
    }

}
