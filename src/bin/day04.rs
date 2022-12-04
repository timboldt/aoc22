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

struct Assignment(i32, i32);

fn parse_assignment(s: &str) -> Assignment {
    let mut iter = s.split('-');
    Assignment(
        iter.next().unwrap().parse::<i32>().unwrap(),
        iter.next().unwrap().parse::<i32>().unwrap(),
    )
}

fn contained_within(a1: &Assignment, a2: &Assignment) -> bool {
    a1.0 <= a2.0 && a1.1 >= a2.1
}

fn overlapping(a1: &Assignment, a2: &Assignment) -> bool {
    (a1.0 >= a2.0 && a1.0 <= a2.1) || (a1.1 >= a2.0 && a1.1 <= a2.1) || (a1.0 < a2.0 && a1.1 > a2.1)
}

fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    let mut result = vec![];
    for line in input.lines() {
        let mut iter = line.split(',');
        result.push((
            parse_assignment(iter.next().unwrap()),
            parse_assignment(iter.next().unwrap()),
        ));
    }
    result
}

fn part1(vals: &[(Assignment, Assignment)]) -> i32 {
    let mut result = 0;
    for v in vals {
        if contained_within(&v.0, &v.1) || contained_within(&v.1, &v.0) {
            result += 1;
        }
    }
    result
}

fn part2(vals: &[(Assignment, Assignment)]) -> i32 {
    let mut result = 0;
    for v in vals {
        if overlapping(&v.0, &v.1) {
            result += 1;
        }
    }
    result
}

fn main() {
    let input = include_str!("../../input/04.txt");
    let parsed = parse(&input);

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
