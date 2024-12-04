pub mod days;

use std::env;

use days::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Day1::interpret_input(&args[1]).run();
    let filename = &args[1];
    day2::Report::run(day2::Report::interpret_input(filename));
}
