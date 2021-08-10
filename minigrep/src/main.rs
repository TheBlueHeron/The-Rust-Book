use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect(); updated: using iterator

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}.", err); // write to stderr using eprintln! macro
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) { // Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
        eprintln!("Application error: {}.", e);

        process::exit(1);
    }
}
