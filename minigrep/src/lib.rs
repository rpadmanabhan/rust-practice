use std::env;
use std::fs;
use std::error::Error;

use kmp;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub use_kmp: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string !"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let use_kmp = !env::var("USE_KMP").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
            use_kmp,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        if config.use_kmp {
            kmp_search(&config.query, &contents)
        }
        else {
            search(&config.query, &contents)
        }
    } else {
        if config.use_kmp {
            kmp_search_case_insensitive(&config.query, &contents)
        }
        else {
            search_case_insensitive(&config.query, &contents)
        }
    };
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn kmp_found(query: &str, line: &str, jump_table: &Vec<usize>) -> bool {
    match kmp::kmp(query, line, &jump_table) {
        Some(_) => true,
        None => false,
    }
}

pub fn kmp_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // index query
    let jump_table:Vec<usize> = kmp::return_failure_function_table(&query);

    // search query in text using index to skip comparisons
    contents
        .lines()
        .filter(|line| kmp_found(query, line, &jump_table))
        .collect()
}

pub fn kmp_search_case_insensitive<'a>(
    query: &str, contents: &'a str) -> Vec<&'a str> {
    // index query
    let jump_table:Vec<usize> = kmp::return_failure_function_table(&query.to_lowercase());

    contents
        .lines()
        .filter(|line| kmp_found(&query.to_lowercase(), &line.to_lowercase(), &jump_table))
        .collect()
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn kmp_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], kmp_search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn kmp_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            kmp_search_case_insensitive(query, contents)
        );
    }

}
