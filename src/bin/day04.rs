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

use std::time::Instant;

struct Assignment {
    low: i32,
    high: i32,
}

impl Assignment {
    fn parse(s: &str) -> Self {
        let mut iter = s.split('-');
        Assignment {
            low: iter.next().unwrap().parse::<i32>().unwrap(),
            high: iter.next().unwrap().parse::<i32>().unwrap(),
        }
    }

    fn contained_within(&self, other: &Assignment) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn overlapping(&self, other: &Assignment) -> bool {
        (self.low >= other.low && self.low <= other.high)
            || (self.high >= other.low && self.high <= other.high)
            || (self.low < other.low && self.high > other.high)
    }
}

fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    let mut result = vec![];
    for line in input.lines() {
        let mut iter = line.split(',');
        result.push((
            Assignment::parse(iter.next().unwrap()),
            Assignment::parse(iter.next().unwrap()),
        ));
    }
    result
}

fn part1(pairs: &[(Assignment, Assignment)]) -> i32 {
    let mut result = 0;
    for pair in pairs {
        let (first, second) = pair;
        if first.contained_within(second) || second.contained_within(first) {
            result += 1;
        }
    }
    result
}

fn part2(pairs: &[(Assignment, Assignment)]) -> i32 {
    let mut result = 0;
    for pair in pairs {
        let (first, second) = pair;
        if first.overlapping(second) {
            result += 1;
        }
    }
    result
}

fn main() {
    let input = include_str!("../../input/04.txt");
    let parsed = parse(input);

    let timer = Instant::now();
    let p1 = part1(&parsed);
    println!("Part 1: {}\n(elapsed: {:.2?})", p1, timer.elapsed());

    let timer = Instant::now();
    let p2 = part2(&parsed);
    println!("Part 2: {}\n(elapsed: {:.2?})", p2, timer.elapsed());
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(2, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(4, super::part2(&input));
    }
}
