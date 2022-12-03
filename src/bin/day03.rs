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
use std::collections::HashSet;
use std::fs;

fn split_rucksack(s: &str) -> (String, String) {
    let s = s.trim();
    let s1 = s[0..s.len() / 2].to_owned();
    let s2 = s[(s.len() / 2)..].to_owned();
    println!("{}/{}", s1, s2);
    (s1, s2)
}

fn common_item(s1: &str, s2: &str) -> char {
    let h1: HashSet<char> = s1.chars().collect();
    let h2: HashSet<char> = s2.chars().collect();
    *(h1.intersection(&h2).into_iter().next().unwrap())
}

fn item_priority(ch: char) -> i32 {
    let v = match ch {
        'a'..='z' => u32::from(ch) - u32::from('a') + 1,
        'A'..='Z' => u32::from(ch) - u32::from('A') + 27,
        _ => 0,
    };
    v.try_into().unwrap()
}

fn parse(input: &str) -> Vec<(String, String)> {
    input.lines().map(split_rucksack).collect()
}

fn part1(vals: &[(String, String)]) -> i32 {
    vals.iter()
        .map(|(s1, s2)| item_priority(common_item(s1, s2)))
        .sum()
}

fn part2(_vals: &[(String, String)]) -> i32 {
    42
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/03.txt")?;
    let parsed = parse(&input);
    let p1 = part1(&parsed);
    let p2 = part2(&parsed);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(157, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(45000, super::part2(&input));
    }
}
