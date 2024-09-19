use std::{env, fs, process};

use ccjson::run;

fn main() {
    let file = fs::read_to_string(env::args().last().unwrap()).unwrap();

    run(&file).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    process::exit(0);
}
