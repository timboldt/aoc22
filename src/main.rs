use std::error::Error;
use std::fmt::Display;
use std::{env, fs};

mod day00;

#[derive(Debug)]
enum AocError {
    MissingArg,
    InvalidDay,
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AocError {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: aoc22 <day>");
        return Err(Box::new(AocError::MissingArg));
    }
    let raw_day: String = args.nth(1).ok_or(Box::new(AocError::InvalidDay))?;
    let day: i32 = raw_day.parse()?;
    let input = fs::read_to_string(format!("input/{:02}.txt", day))?;

    println!("== Day {} ==", day);
    match day {
        0 => day00::run(input),
        _ => println!("Invalid day: {}", raw_day),
    };

    Ok(())
}
