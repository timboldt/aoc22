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
use std::fs;
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|s| s.parse()).collect()
}

fn part1(vals: &[i32]) -> i32 {
    vals.iter().sum()
}

fn part2(vals: &[i32]) -> i32 {
    let subtot: i32 = vals.iter().sum();
    subtot * 2
}

fn main() -> Result<()> {
    let input = fs::read_to_string(format!("input/02.txt"))?;
    let parsed = parse(&input)?;
    let p1 = part1(&parsed);
    let p2 = part2(&parsed);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() {
        assert_eq!(1 + 2 + 3, super::part1(&[1, 2, 3]));
    }

    #[test]
    fn part2_works() {
        assert_eq!(2 + 4 + 6, super::part2(&[1, 2, 3]));
    }
}
