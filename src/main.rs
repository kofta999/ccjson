use std::{env, fs};

use ccjson::run;

fn main() {
    let file = fs::read_to_string(env::args().last().unwrap()).unwrap();

    run(&file);
}
