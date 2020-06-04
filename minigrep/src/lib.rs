use std::env;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::io::prelude::BufRead;

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

    let f = File::open(config.filename)?;
    let reader = BufReader::new(f);

    let results = if config.case_sensitive {
        if config.use_kmp {
            kmp_search(&config.query, reader)
        }
        else {
            search(&config.query, reader)
        }
    } else {
        if config.use_kmp {
            kmp_search_case_insensitive(&config.query, reader)
        }
        else {
            search_case_insensitive(&config.query, reader)
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

pub fn kmp_search<T: BufRead + Sized>(query: &str, reader: T) -> Vec<String> {
    // index query
    let jump_table:Vec<usize> = kmp::return_failure_function_table(&query);

    let mut result = Vec::new();

    // search query in text using index to skip comparisons
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if kmp_found(query, &line, &jump_table)  {
            result.push(line);
        }
    }

    result
}

pub fn kmp_search_case_insensitive<T: BufRead + Sized>(query: &str, reader: T) -> Vec<String> {
    // index query
    let jump_table:Vec<usize> = kmp::return_failure_function_table(&query.to_lowercase());

    let mut result = Vec::new();

    // search query in text using index to skip comparisons
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if kmp_found(&query.to_lowercase(), &line.to_lowercase(), &jump_table)  {
            result.push(line);
        }
    }

    result

}


pub fn search<T: BufRead + Sized>(query: &str, reader: T) -> Vec<String> {
    let mut result = Vec::new();

    // search query in text using index to skip comparisons
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.contains(query)  {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<T: BufRead + Sized>(query: &str, reader: T) -> Vec<String> {
    let mut result = Vec::new();

    // search query in text using index to skip comparisons
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.to_lowercase().contains(&query.to_lowercase())  {
            result.push(line);
        }
    }

    result
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
