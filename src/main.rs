//  Copyright 2022 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#![warn(clippy::all)]

use std::error::Error;
use std::fmt::Display;
use std::{env, fs};

mod day00;

#[derive(Debug)]
enum AocError {
    MissingArg,
    InvalidDay(String),
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
    let raw_day: String = args.nth(1).ok_or_else(|| Box::new(AocError::MissingArg))?;
    let day: i32 = raw_day.parse()?;
    let input = fs::read_to_string(format!("input/{:02}.txt", day))?;

    let (p1, p2) = match day {
        0 => day00::run(&input),
        _ => {
            let e: Box<dyn Error> = Box::new(AocError::InvalidDay(raw_day));
            Err(e)
        }
    }?;

    println!("== Day {} ==", day);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}
