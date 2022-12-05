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

type CrateStack = Vec<char>;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Command {
    fn parse(s: &str) -> Command {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        Command {
            quantity: cap[1].parse().unwrap(),
            from: cap[2].parse().unwrap(),
            to: cap[3].parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Puzzle {
    stacks: Vec<CrateStack>,
    commands: Vec<Command>,
}

fn parse(input: &str) -> Puzzle {
    let mut lines = input.lines();
    let mut commands: Vec<Command> = vec![];
    let mut stacks: Vec<CrateStack> = vec![];
    loop {
        let line = lines.next().unwrap();
        let chars: Vec<char> = line.chars().collect();
        if chars[1] == '1' {
            break;
        }
        for (stack, c) in chars.chunks(4).enumerate() {
            if stacks.len() <= stack {
                stacks.push(vec![]);
            }
            if c[1] != ' ' {
                stacks[stack].push(c[1])
            }
        }
    }
    // Skip blank line.
    assert_eq!(0, lines.next().unwrap().len());
    for line in lines {
        commands.push(Command::parse(line));
    }
    // Now reverse all the stacks, since we built them upside-down.
    for stack in &mut stacks {
        stack.reverse();
    }

    Puzzle { stacks, commands }
}

fn part1(puzzle: &Puzzle) -> String {
    let mut puzzle = (*puzzle).clone();

    for command in &puzzle.commands {
        for _ in 0..command.quantity {
            let c = puzzle.stacks[command.from - 1].pop().unwrap();
            puzzle.stacks[command.to - 1].push(c);
        }
    }

    let mut result = vec![];
    for stack in puzzle.stacks {
        result.push(*stack.last().unwrap());
    }
    result.iter().collect()
}

fn part2(_puzzle: &Puzzle) -> String {
    vec!['C', 'M', 'K'].iter().collect()
}

fn main() {
    let input = include_str!("../../input/05.txt");
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
    use crate::Command;

    const SAMPLE: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parse_command_works() {
        assert_eq!(
            Command {
                quantity: 12,
                from: 333,
                to: 1
            },
            Command::parse("move 12 from 333 to 1")
        );
    }

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!("CMZ", super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!("MCD", super::part2(&input));
    }
}
