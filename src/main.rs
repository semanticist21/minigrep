use std::{env, process};

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap();

    if let Err(_) = minigrep::run(&config) {
        exit_str("erorr has occured during running scanning");
    };
}

fn exit_str(_: &str) {
    process::exit(1);
}
