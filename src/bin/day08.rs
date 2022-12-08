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

fn is_visible(puzzle: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    if row == 0 || row == puzzle.len() - 1 {
        return true;
    }
    if col == 0 || col == puzzle[row].len() - 1 {
        return true;
    }
    let target = puzzle[row][col];
    let mut visible;

    visible = true;
    for r in 0..row {
        if puzzle[r][col] >= target {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for r in row + 1..puzzle.len() {
        if puzzle[r][col] >= target {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for c in 0..col {
        if puzzle[row][c] >= target {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for c in col + 1..puzzle[row].len() {
        if puzzle[row][c] >= target {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    false
}

fn viewing_score(puzzle: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let target = puzzle[row][col];

    let mut left = 0;
    for r in (0..row).rev() {
        left += 1;
        if puzzle[r][col] >= target {
            break;
        }
    }

    let mut right = 0;
    for r in row + 1..puzzle.len() {
        right += 1;
        if puzzle[r][col] >= target {
            break;
        }
    }

    let mut top = 0;
    for c in (0..col).rev() {
        top += 1;
        if puzzle[row][c] >= target {
            break;
        }
    }

    let mut bottom = 0;
    for c in col + 1..puzzle[row].len() {
        bottom += 1;
        if puzzle[row][c] >= target {
            break;
        }
    }

    left * right * top * bottom
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut forest: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap() as u8);
        }
        forest.push(row);
    }
    forest
}

fn part1(puzzle: &Vec<Vec<u8>>) -> usize {
    let rows = puzzle.len();
    let cols = puzzle[0].len();
    let mut visible: usize = 0;
    for row in 0..rows {
        for col in 0..cols {
            if is_visible(puzzle, row, col) {
                visible += 1;
            }
        }
    }
    visible
}

fn part2(puzzle: &Vec<Vec<u8>>) -> usize {
    let rows = puzzle.len();
    let cols = puzzle[0].len();
    let mut best: usize = 0;
    for row in 0..rows {
        for col in 0..cols {
            let score = viewing_score(puzzle, row, col);
            if score > best {
                best = score;
            }
        }
    }
    best
}

fn main() {
    let input = include_str!("../../input/08.txt");
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
    const SAMPLE: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(21, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(8, super::part2(&input));
    }
}
