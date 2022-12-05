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
use itertools::Itertools;
use std::collections::HashSet;
use std::time::Instant;

fn split_rucksack(s: &str) -> (String, String) {
    let s1 = s[0..s.len() / 2].to_owned();
    let s2 = s[(s.len() / 2)..].to_owned();
    (s1, s2)
}

fn common_item(s1: &str, s2: &str) -> char {
    let h1: HashSet<char> = s1.chars().collect();
    let h2: HashSet<char> = s2.chars().collect();
    *(h1.intersection(&h2).into_iter().next().unwrap())
}

fn common_item_among_three(s1: &str, s2: &str, s3: &str) -> char {
    let h1: HashSet<char> = s1.chars().collect();
    let h2: HashSet<char> = s2.chars().collect();
    // TODO: Horrible hack. There must be a way to collect directly into a HashSet.
    let s1_2: String = h1.intersection(&h2).collect();
    let h1_2: HashSet<char> = s1_2.chars().collect();
    let h3: HashSet<char> = s3.chars().collect();
    *(h1_2.intersection(&h3).into_iter().next().unwrap())
}

fn item_priority(ch: char) -> i32 {
    let v = match ch {
        'a'..='z' => u32::from(ch) - u32::from('a') + 1,
        'A'..='Z' => u32::from(ch) - u32::from('A') + 27,
        _ => 0,
    };
    v.try_into().unwrap()
}

fn parse1(input: &str) -> Vec<(String, String)> {
    input.lines().map(split_rucksack).collect()
}

fn part1(vals: &[(String, String)]) -> i32 {
    vals.iter()
        .map(|(s1, s2)| item_priority(common_item(s1, s2)))
        .sum()
}

fn parse2(input: &str) -> Vec<(String, String, String)> {
    let mut result: Vec<(String, String, String)> = vec![];
    for chunk in &input.lines().chunks(3) {
        let (x, y, z) = chunk.collect_tuple().unwrap();
        result.push((x.to_owned(), y.to_owned(), z.to_owned()));
    }
    result
}

fn part2(vals: &[(String, String, String)]) -> i32 {
    vals.iter()
        .map(|(s1, s2, s3)| item_priority(common_item_among_three(s1, s2, s3)))
        .sum()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/03.txt");

    let parsed = parse1(input);

    let timer = Instant::now();
    let p1 = part1(&parsed);
    println!("Part 1: {}\n(elapsed: {:.2?})", p1, timer.elapsed());

    let parsed = parse2(input);

    let timer = Instant::now();
    let p2 = part2(&parsed);
    println!("Part 2: {}\n(elapsed: {:.2?})", p2, timer.elapsed());

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
        let input = super::parse1(SAMPLE);
        assert_eq!(157, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse2(SAMPLE);
        assert_eq!(70, super::part2(&input));
    }
}
