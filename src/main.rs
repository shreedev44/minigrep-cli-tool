use minigrep_cli_tool::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Applciation error: {e}");
        process::exit(1)
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path "),
        };

        let ignore_case_argument = match args.next() {
            Some(value) if value == "/i" => Some(true),
            Some(value) if value == "/s" => Some(false),
            _ => None,
        };

        let ignore_case = match ignore_case_argument {
            Some(value) => value,
            None => env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        search_case_insensitive(&config.query, &contents).for_each(|line| println!("{line}"));
    } else {
        search(&config.query, &contents).for_each(|line| println!("{line}"));
    };
    Ok(())
}
