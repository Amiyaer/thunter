use core::panic;
use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };
    for (idx,c) in result.iter().enumerate() {
        println!("{}||||-=-=-=-=-||||{}",idx,c);
    }
    // println!("With text:\n{contents}");
    Ok(())
}

fn search<'a>(target: &str, content: &'a str) -> Vec<&'a str> {
    println!("search sensitive");
    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(target) {
            res.push(line);
        }
    }
    res
}

fn search_insensitive<'a>(target: &str, content: &'a str) -> Vec<&'a str> {
    println!("search insensitive");
    let target = target.to_lowercase();
    let mut res = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&target) {
            res.push(line);
        }
    }
    res
}

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config { query, file_path, ignore_case }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").
            map_or(false, |var| var.eq("1"));

        // let ignore_case = true;
        Ok(Config { query, file_path, ignore_case })
    }
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
        println!("case sensitive");
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            vec!["Rust:", "Trust me."], search_insensitive(query, contents));
    }
}