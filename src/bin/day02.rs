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

use anyhow::{anyhow, Result};

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

struct GameRound {
    them: Hand,
    us: Hand,
}

fn parse_line(line: &str) -> Result<GameRound> {
    let mut iter = line.split_whitespace();
    Ok(GameRound {
        them: match iter.next() {
            Some("A") => Hand::Rock,
            Some("B") => Hand::Paper,
            Some("C") => Hand::Scissors,
            _ => return Err(anyhow!("invalid line: {}", line)),
        },
        us: match iter.next() {
            Some("X") => Hand::Rock,
            Some("Y") => Hand::Paper,
            Some("Z") => Hand::Scissors,
            _ => return Err(anyhow!("invalid line: {}", line)),
        },
    })
}

fn score(gr: &GameRound) -> i32 {
    match gr.us {
        Hand::Rock => match gr.them {
            Hand::Rock => 1 + 3,
            Hand::Paper => 1,
            Hand::Scissors => 1 + 6,
        },
        Hand::Paper => match gr.them {
            Hand::Rock => 2 + 6,
            Hand::Paper => 2 + 3,
            Hand::Scissors => 2,
        },
        Hand::Scissors => match gr.them {
            Hand::Rock => 3,
            Hand::Paper => 3 + 6,
            Hand::Scissors => 3 + 3,
        },
    }
}

fn reinterpret_part2(gr: &GameRound) -> GameRound {
    GameRound {
        them: gr.them,
        us: match gr.us {
            Hand::Rock => match gr.them {
                // We want to lose.
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },
            Hand::Paper => gr.them,
            Hand::Scissors => match gr.them {
                // We want to win.
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
        },
    }
}

fn parse(input: &str) -> Result<Vec<GameRound>> {
    input.lines().map(parse_line).collect()
}

fn part1(vals: &[GameRound]) -> i32 {
    vals.iter()
        .map(score)
        .reduce(|accum, item| accum + item)
        .unwrap_or_default()
}

fn part2(vals: &[GameRound]) -> i32 {
    vals.iter()
        .map(reinterpret_part2)
        .map(|gr| score(&gr))
        .reduce(|accum, item| accum + item)
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/02.txt");
    let parsed = parse(&input)?;
    let p1 = part1(&parsed);
    let p2 = part2(&parsed);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE).unwrap();
        assert_eq!(15, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE).unwrap();
        assert_eq!(12, super::part2(&input));
    }
}
