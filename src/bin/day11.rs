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

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Monkey {
    num_inspections: usize,
    items: Vec<usize>,
    op: MonkeyOp,
    modulus: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone, Copy)]
enum MonkeyOp {
    Plus(usize),
    Times(usize),
    Square,
}

fn parse(input: &str) -> Vec<Monkey> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?ms)Monkey\s*(\d+).*$
\s*Starting items:\s*(.*)$
\s*Operation: new = old (.) (.+)$
\s*Test: divisible by (\d+)$
\s*If true: throw to monkey (\d+)$
\s*If false: throw to monkey (\d+)$"
        )
        .unwrap();
    }

    let mut monkeys: Vec<Monkey> = Vec::new();
    for entry in input.split("\n\n") {
        let cap = RE.captures(entry).unwrap();
        monkeys.push(Monkey {
            num_inspections: 0,
            items: cap[2]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect(),
            op: match (&cap[3], &cap[4]) {
                ("+", v) => MonkeyOp::Plus(v.trim().parse().unwrap()),
                ("*", "old") => MonkeyOp::Square,
                ("*", v) => MonkeyOp::Times(v.trim().parse().unwrap()),
                _ => unreachable!(),
            },
            modulus: cap[5].parse().unwrap(),
            if_true: cap[6].parse().unwrap(),
            if_false: cap[7].parse().unwrap(),
        });
    }
    println!("{:?}", monkeys);
    monkeys
}

fn part1(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.iter().cloned().collect_vec();
    for round in 0..20 {
        for m in 0..monkeys.len() {
            while let Some(w) = monkeys[m].items.pop() {
                let worry = match monkeys[m].op {
                    MonkeyOp::Plus(x) => w + x,
                    MonkeyOp::Times(x) => w * x,
                    MonkeyOp::Square => w * w,
                } / 3;
                let target = if worry % monkeys[m].modulus == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys[target].items.push(worry);
                monkeys[m].num_inspections += 1;
            }
        }
        println!("\n\nRound {}: {:?}", round, monkeys);
    }
    // Reverse sort.
    monkeys.sort_by(|b, a| a.num_inspections.cmp(&b.num_inspections));
    monkeys.iter().take(2).map(|m| m.num_inspections).product()
}

fn part2(_monkeys: &[Monkey]) -> i32 {
    0
}

fn main() {
    let input = include_str!("../../input/11.txt");
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
    const SAMPLE: &str = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(10605, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(36, super::part2(&input));
    }
}
