
use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search,search_case_insensitive};

fn main() {

    //env::args() returns an iterator with type String.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
   }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Remember that ? returns the error value from the current function
    //for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    //calling run for side ffects only, returns unit type to satisfy Result
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(
        mut args: impl Iterator<Item=String>
        ) -> Result<Config, &'static str> {

        //let mut no_env = String::new();

        //ignore name of program and consume
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path string"),
        };


        //Optional command, hence return a string.
        //any string will suffice to delete the environment variable
        //"IGNORE_CASE".
        let no_env = match args.next() {
            Some(arg) => arg,
            None => String::new(),
        };

        //env::remove_var is idempotent, so it can succeed infinitely
        //with or without the existence of "IGNORE_CASE"
        if !no_env.is_empty() {
            unsafe {
                env::remove_var("IGNORE_CASE");
            }
        }

        //to set this in bash:
        // ~/$IGNORE_CASE=1 cargo run -- ... ...
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }

}

