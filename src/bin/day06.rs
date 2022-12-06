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

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(puzzle: &[char]) -> i32 {
    let mut marker: [char; 4] = puzzle[0..4].try_into().unwrap();
    for (idx, ch) in puzzle.iter().enumerate() {
        marker[idx % 4] = *ch;
        let mut dup = false;
        for i in 0..4 {
            for j in i + 1..4 {
                if marker[i] == marker[j] {
                    dup = true;
                }
            }
        }
        if !dup {
            return idx as i32 + 1;
        }
    }
    0
}

fn part2(puzzle: &[char]) -> i32 {
    let mut marker: [char; 14] = puzzle[0..14].try_into().unwrap();
    for (idx, ch) in puzzle.iter().enumerate() {
        marker[idx % 14] = *ch;
        let mut dup = false;
        for i in 0..14 {
            for j in i + 1..14 {
                if marker[i] == marker[j] {
                    dup = true;
                }
            }
        }
        if !dup {
            return idx as i32 + 1;
        }
    }
    0
}

fn main() {
    let input = include_str!("../../input/06.txt");
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
    const SAMPLE: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(7, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(19, super::part2(&input));
    }
}
