use std::{env, process};
use file_search::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    if let Err(e) = file_search::run(config) {
        // 错误输出到控制台
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
