//! # Minigrep CLI Tool
//! 
//! A command-line utility similar to classic 'grep' tool. 
//! It searches for lines containing a specified query string withing a text file.
//! 
//! This crate supports both **case-sensitive** and **case-insensitive** searches,
//! controlled via a command-line flag or an environment variable.
//! 
//! 
//! # Usage
//! ```
//! cargo run -- <query> <file_path> <flag> [/i or /s]
//! ```
//! 
//! - '/i' enables case-insensitive search
//! - '/s' enables case-sensitive search
//! 
//! Alternatively, you can enable case-insensitive search using the environment variable
//! ```
//! IGNORE_CASE=1 cargo run -- <query> <file_path>
//! ```
//! 
//! Example:
//! ```
//! cargo run -- rust docs.txt /i
//! ```


use minigrep_cli_tool::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

///The entry point of the Minigrep CLI Tool.
/// 
/// Parses command-line arguments, builds the configuration,
/// and runs the main search routine. Any errors during argument
/// parsing or execution display a message and terminate the process
/// with a non-zero exit code.

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

/// Holds the command-line configuration for the program.
///
/// - `query`: The substring to search for.
/// - `file_path`: Path to the file to search.
/// - `ignore_case`: If `true`, performs a case-insensitive search.

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    /// Builds a new `Config` instance from command-line arguments.
    ///
    /// Expected argument format:
    /// ```
    /// minigrep <query> <file_path> [/i or /s]
    /// ```
    ///
    /// - `/i` sets `ignore_case` to true
    /// - `/s` sets `ignore_case` to false
    /// - If no flag is provided, the environment variable `IGNORE_CASE`
    ///   determines behavior.
    ///
    /// # Errors
    /// Returns an error if either query or file path is missing.
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

/// Executes the search process.
///
/// Reads the file indicated by the config, performs the search
/// (case-sensitive or insensitive), and prints all matching lines.
///
/// # Errors
/// Returns a boxed error if the file cannot be read.

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        search_case_insensitive(&config.query, &contents).for_each(|line| println!("{line}"));
    } else {
        search(&config.query, &contents).for_each(|line| println!("{line}"));
    };
    Ok(())
}
