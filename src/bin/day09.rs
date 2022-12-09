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

use std::{time::Instant, collections::HashSet};

enum Step {
    Left(u8),
    Right(u8),
    Up(u8),
    Down(u8),
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);

fn follow_head(head: Point, tail: Point) -> Point {
    let mut result = tail;
    let mut moved_x = false;
    let mut moved_y = false;
    if head.0 - 1 > tail.0 {
        result.0 = head.0 - 1;
        moved_x = true;
    } else if head.0 + 1 < tail.0 {
        result.0 = head.0 + 1;
        moved_x = true;
    }
    if head.1 - 1 > tail.1 {
        result.1 = head.1 - 1;
        moved_y = true;
    } else if head.1 + 1 < tail.1 {
        result.1 = head.1 + 1;
        moved_y = true;
    }
    if moved_x && !moved_y {
        result.1 = head.1;
    } else if moved_y && !moved_x {
        result.0 = head.0;
    }
    result
}

fn parse(input: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap().chars().next().unwrap();
        let distance = parts.next().unwrap().parse().unwrap();
        steps.push(match direction {
            'L' => Step::Left(distance),
            'R' => Step::Right(distance),
            'U' => Step::Up(distance),
            'D' => Step::Down(distance),
            _ => unreachable!(),
        })
    }
    steps
}

fn part1(steps: &[Step]) -> usize {
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);
    let mut visited = HashSet::new();
    for step in steps {
        match step {
            Step::Left(distance) => {
                for i in 0..*distance {
                    head = Point(head.0 - 1, head.1);
                    tail = follow_head(head, tail);
                    visited.insert(tail);
                }
            }
            Step::Right(distance) => {
                for i in 0..*distance {
                    head = Point(head.0 + 1, head.1);
                    tail = follow_head(head, tail);
                    visited.insert(tail);
                }
            }
            Step::Up(distance) => {
                for i in 0..*distance {
                    head = Point(head.0, head.1 + 1);
                    tail = follow_head(head, tail);
                    visited.insert(tail);
                }
            }
            Step::Down(distance) => {
                for i in 0..*distance {
                    head = Point(head.0, head.1 - 1);
                    tail = follow_head(head, tail);
                    visited.insert(tail);
                }
            }
        }
        println!("{:?} {:?}", head, tail);
    }
    visited.len()
}

fn part2(_steps: &[Step]) -> usize {
    42
}

fn main() {
    let input = include_str!("../../input/09.txt");
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
    const SAMPLE: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(13, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(8, super::part2(&input));
    }
}
