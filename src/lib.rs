use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    //println!("with text: \n{}", contents);
    search(&config.query, &contents, !config.case_sensitive).into_iter()
        .inspect(|&line| println!("{}", line)).collect::<Vec<_>>();

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args)-> Result<Config, &'static str> {
        args.next(); // trash

        let query = args.next().ok_or("Didn't get a query string")?;
        let filename = args.next().ok_or("Didn't get a file name")?;
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config {
                query: query,
                filename: filename,
                case_sensitive: case_sensitive,
            }
        )
    }
}

pub fn search<'a>(query: &str, contents: &'a str, is_case_insensitive: bool)-> Vec<&'a str> {
    let new_query = if is_case_insensitive {query.to_lowercase()} else {query.to_owned()};
    contents.lines()
        .filter_map(|line| {
            let new_line = if is_case_insensitive {line.to_lowercase()} else {line.to_owned()};
            if new_line.contains(&new_query) { Some(line) } else { None }
        })
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
        search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productiive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, true)
        );

    }
}