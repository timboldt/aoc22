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

use std::{fmt::Result, time::Instant};

use itertools::Itertools;

fn parse(input: &str) -> Vec<(String, String)> {
    let mut parsed = vec![];
    for mut chunk in &input.lines().chunks(3) {
        let left = chunk.next().unwrap();
        let right = chunk.next().unwrap();
        parsed.push((left.to_owned(), right.to_owned()));
    }
    parsed
}

fn in_correct_order(left: &str, right: &str) -> bool {
    // TODO: Implement me.
    true
}

fn part1(parsed: &[(String, String)]) -> usize {
    let mut result: usize = 0;
    for (idx, (left, right)) in parsed.iter().enumerate() {
        if in_correct_order(left, right) {
            result += idx + 1;
        }
    }
    result
}

fn part2(parsed: &[(String, String)]) -> usize {
    42
}

fn main() {
    let input = include_str!("../../input/13.txt");
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
    const SAMPLE: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(13, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(29, super::part2(&input));
    }
}
