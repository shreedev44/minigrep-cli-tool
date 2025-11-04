pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.contains(query))
}

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
