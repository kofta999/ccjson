use std::{env, fs, process};

use ccjson::run;

fn main() {
    let mut args = env::args();
    args.next();

    let file = match args.next() {
        Some(arg) => fs::read_to_string(arg).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        }),
        None => {
            eprintln!("No arguments provided");
            process::exit(1);
        }
    };

    run(&file).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    process::exit(0);
}
