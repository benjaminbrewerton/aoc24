pub mod day1;
pub mod day2;

pub trait AOCDaySingle<Rhs=Self> {
    fn interpret_input(file: &str) -> Rhs;

    fn run(&self) {
        println!("Default hello!")
    }
}

pub trait AOCDay<Rhs=Self> {
    fn interpret_input(file: &str) -> Rhs;

    fn run(items: Rhs);
}