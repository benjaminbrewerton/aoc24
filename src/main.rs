pub mod days;

use std::{env, ops::Index};

use days::{day1::Day1, AOCDay};

fn main() {
    let args: Vec<String> = env::args().collect();
    Day1::interpret_input(&args[1]).run();
}
