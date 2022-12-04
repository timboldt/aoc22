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

use anyhow::Result;
use regex::Regex;
use std::fs;
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    Regex::new(r"\n[ \t]*\n")
        .unwrap()
        .split(input)
        .map(|elf| elf.split_whitespace().map(|s| s.parse::<i32>()).sum())
        .collect()
}

fn part1(vals: &[i32]) -> i32 {
    *vals
        .iter()
        .reduce(|accum, item| if *item > *accum { item } else { accum })
        .unwrap_or(&0)
}

fn part2(vals: &[i32]) -> i32 {
    let mut mut_vals = vals.to_vec();
    mut_vals.sort();
    mut_vals[mut_vals.len() - 3..].iter().sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/01.txt")?;
    let parsed = parse(&input)?;
    let p1 = part1(&parsed);
    let p2 = part2(&parsed);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE).unwrap();
        assert_eq!(24000, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE).unwrap();
        assert_eq!(45000, super::part2(&input));
    }
}
