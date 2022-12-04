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

fn parse(input: &str) -> Vec<i32> {
    let mut result = vec![];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    result.push(sum);
    result
}

fn part1(vals: &[i32]) -> i32 {
    let mut result = 0;
    for v in vals {
        if *v > result {
            result = *v;
        }
    }
    result
}

fn part2(vals: &[i32]) -> i32 {
    let mut top = [0, 0, 0];
    for v in vals {
        let mut v = *v;
        for t in top.iter_mut() {
            if v > *t {
                let tmp = *t;
                *t = v;
                v = tmp;
            }
        }
    }
    let mut result = 0;
    for t in top {
        result += t;
    }
    result
}

fn main() {
    let input = include_str!("../../input/01.txt");
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
        let input = super::parse(SAMPLE);
        assert_eq!(24000, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(45000, super::part2(&input));
    }
}
