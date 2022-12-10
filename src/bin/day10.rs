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

enum Op {
    NoOp,
    AddX(i32),
}

fn cycle_count(op: &Op) -> i32 {
    match op {
        Op::NoOp => 1,
        Op::AddX(_) => 2,
    }
}

fn parse(input: &str) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        ops.push(match parts.next().unwrap() {
            "noop" => Op::NoOp,
            "addx" => Op::AddX(parts.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        })
    }
    ops
}

fn part1(ops: &[Op]) -> i32 {
    let mut result = 0;
    let mut signal = 1;
    let mut cycles = 0;
    for op in ops {
        for _ in 0..cycle_count(op) {
            cycles += 1;
            if (cycles + 20) % 40 == 0 {
                result += signal * cycles;
            }
        }
        match *op {
            Op::AddX(s) => signal += s,
            _ => {}
        }
        if cycles >= 220 {
            break;
        }
    }
    result
}

fn part2(ops: &[Op]) -> i32 {
    let mut signal = 1;
    let mut cycles = 0;
    for op in ops {
        for _ in 0..cycle_count(op) {
            let pos = cycles % 40;
            if pos == 0 {
                println!();
            }
            if signal >= pos - 1 && signal <= pos + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycles += 1;
        }
        match *op {
            Op::AddX(s) => signal += s,
            _ => {}
        }
    }
    println!();
    println!();
    0
}

fn main() {
    let input = include_str!("../../input/10.txt");
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
    const SAMPLE: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(13140, super::part1(&input));
    }

    // #[test]
    // fn part2_works() {
    //     let input = super::parse(SAMPLE);
    //     assert_eq!(36, super::part2(&input));
    // }
}
