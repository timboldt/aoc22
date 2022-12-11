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

use lazy_static::lazy_static;
use regex::Regex;
use std::time::Instant;

fn parse(input: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?ms)Monkey\s*(\d+).*$
\s*Starting items:\s*(.*)$
\s*Operation: new = old (.) (.+)$
\s*Test: divisible by (\d+)$
\s*If true: throw to monkey (\d+)$
\s*If false: throw to monkey (\d+)$"
        )
        .unwrap();
    }
    for entry in input.split("\n\n") {
        let cap = RE.captures(entry).unwrap();
        println!(
            "M:{} S:{} O:{}{} D:{} MT:{} MF:{}",
            &cap[1], &cap[2], &cap[3], &cap[4], &cap[5], &cap[6], &cap[7]
        );
    }

    let mut result: Vec<i32> = Vec::new();
    result
}

fn part1(parsed: &[i32]) -> i32 {
    42
}

fn part2(parsed: &[i32]) -> i32 {
    0
}

fn main() {
    let input = include_str!("../../input/11.txt");
    let parsed = parse(input);

    let timer = Instant::now();
    let p1 = part1(&parsed);
    println!("Part 1: {:?}\n(elapsed: {:.2?})", p1, timer.elapsed());

    let timer = Instant::now();
    let p2 = part2(&parsed);
    println!("Part 2: {:?}\n(elapsed: {:.2?})", p2, timer.elapsed());
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(13140, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(36, super::part2(&input));
    }
}
