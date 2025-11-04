//! # minigrep_cli_tool
//!
//! A lightweight library module that powers the MiniGrep CLI tool.
//!
//! It provides two main functions for searching within text:
//! - `search` (case-sensitive)
//! - `search_case_insensitive` (case-insensitive)
//!
//! # Examples
//! ```
//! use minigrep_cli_tool::{search, search_case_insensitive};
//!
//! let query = "rust";
//! let contents = "Rust is fast.\nTrust in Rust.";
//!
//! // Case-sensitive
//! let matches: Vec<&str> = search(query, contents).collect();
//!
//! // Case-insensitive
//! let matches_insensitive: Vec<&str> = search_case_insensitive(query, contents).collect();
//! ```

/// Searches for lines containing the query string in the provided text.
///
/// This function performs a **case-sensitive** search.
///
/// # Arguments
/// - `query`: The substring to look for.
/// - `contents`: The text to search within.
///
/// # Returns
/// An iterator over lines that contain the query.
///
/// # Examples
/// ```
/// use minigrep_cli_tool::search;
///
/// let query = "safe";
/// let contents = "Rust is safe.\nFast.\nProductive.";
///
/// let results: Vec<&str> = search(query, contents).collect();
/// assert_eq!(results, vec!["Rust is safe."]);
/// ```

pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.contains(query))
}

/// Searches for lines containing the query string, ignoring case.
///
/// # Arguments
/// - `query`: The substring to look for.
/// - `contents`: The text to search within.
///
/// # Returns
/// An iterator over lines that contain the query, ignoring case.
///
/// # Examples
/// ```
/// use minigrep_cli_tool::search_case_insensitive;
///
/// let query = "RuSt";
/// let contents = "Rust:\nReally productive.\nTrust in rust.";
///
/// let results: Vec<&str> = search_case_insensitive(query, contents).collect();
/// assert_eq!(results, vec!["Rust:", "Trust in rust."]);
/// ```

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let result: Vec<&str> = search(query, contents).collect();
        assert_eq!(result, vec!["safe, fast, productive."]);
    }

    #[test]
    fn multiple_result() {
        let query = "ive";
        let contents = "\
Rust:
really productive.
also passive.
probably problamatic.
but simply lovely.
Come dive into the world of rust.";

        let result: Vec<&str> = search(query, contents).collect();
        assert_eq!(
            result,
            vec![
                "really productive.",
                "also passive.",
                "Come dive into the world of rust."
            ]
        )
    }

    #[test]
    fn empty_content() {
        let query = "hi";
        let contents = "";

        let result: Vec<&str> = search(query, contents).collect();
        let expected: Vec<&str> = Vec::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn empty_query() {
        let query = "";
        let contents = "\
Rust:
really productive.
also passive.
probably problamatic.
but simply lovely.
Come dive into the world of rust.";

        let result: Vec<&str> = search(query, contents).collect();
        let expected: Vec<&str> = contents.lines().collect();
        assert_eq!(result, expected)
    }

    #[test]
    fn empty_content_and_query() {
        let query = "";
        let contents = "";

        let result: Vec<&str> = search(query, contents).collect();
        let expected: Vec<&str> = Vec::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
really productive.
also passive.
probably problamatic.
but simply lovely.
Come dive into the world of rust.";

        let result: Vec<&str> = search_case_insensitive(query, contents).collect();
        assert_eq!(result, vec!["Rust:", "Come dive into the world of rust."]);
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
really productive.
also passive.
probably problamatic.
but simply lovely.
Come dive into the world of rust.";

        let result: Vec<&str> = search(query, contents).collect();
        let expected: Vec<&str> = Vec::new();
        assert_eq!(result, expected);
    }
}
