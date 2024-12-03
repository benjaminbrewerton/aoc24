pub mod day1;

pub trait AOCDay {
    fn interpret_input(file: &str) -> Self;

    fn run(&self) {
        println!("Default hello!")
    }
}