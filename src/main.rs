extern crate twitter_cli;

use std::env;
use std::process;

use twitter_cli::Config;

fn main() {
    let message = read_message(env::args()).unwrap_or_else(|err| {
        println!("Error while reading your tweet: {}", err);
        process::exit(1);
    });

    let config = Config::new().unwrap_or_else(|err| {
        println!("Error while initializing configs: {}", err);
        process::exit(1);
    });

    twitter_cli::run(message, config).unwrap_or_else(|err| {
        println!("Runtime error: {}", err);
        process::exit(1);
    });
}

fn read_message(mut args: env::Args) -> Result<String, &'static str> {
    match args.nth(1) {
        Some(tweet) => Ok(tweet),
        None => Err("Not given a tweet"),
    }
}
