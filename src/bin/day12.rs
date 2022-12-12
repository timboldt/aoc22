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

use array2d::Array2D;
use pathfinding::prelude::bfs;

// These are derived from the known actual input, plus a border wall.
const MAX_ROWS: usize = 43;
const MAX_COLS: usize = 164;

struct Map {
    start: (usize, usize),
    end: (usize, usize),
    heights: Array2D<u8>,
}

fn parse(input: &str) -> Map {
    let mut out = Map {
        start: (0, 0),
        end: (0, 0),
        heights: Array2D::filled_with(u8::MAX, MAX_ROWS, MAX_COLS),
    };
    for (r, line) in input.lines().enumerate() {
        let row = r + 1; // Leave a border wall.
        for (c, ch) in line.bytes().enumerate() {
            let col = c + 1; // Leave a border wall.
            match ch {
                b'a'..=b'z' => out.heights[(row, col)] = ch - b'a',
                b'S' => {
                    out.heights[(row, col)] = 0;
                    out.start = (row, col);
                }
                b'E' => {
                    out.heights[(row, col)] = b'z' - b'a';
                    out.end = (row, col);
                }
                _ => unreachable!(),
            }
        }
    }
    out
}

fn can_travel(from: u8, to: u8) -> bool {
    to <= from + 1
}

fn reachable(map: &Map, current: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut out = vec![];
    let (row, col) = *current;
    if can_travel(map.heights[(row, col)], map.heights[(row - 1, col)]) {
        out.push((row - 1, col));
    }
    if can_travel(map.heights[(row, col)], map.heights[(row + 1, col)]) {
        out.push((row + 1, col));
    }
    if can_travel(map.heights[(row, col)], map.heights[(row, col - 1)]) {
        out.push((row, col - 1));
    }
    if can_travel(map.heights[(row, col)], map.heights[(row, col + 1)]) {
        out.push((row, col + 1));
    }
    out
}

fn part1(map: &Map) -> usize {
    let result = bfs(&map.start, |p| reachable(map, p), |p| *p == map.end).unwrap();
    result.len() - 1
}

fn part2(map: &Map) -> usize {
    let mut best = usize::MAX;
    for r in 0..MAX_ROWS {
        for c in 0..MAX_COLS {
            if map.heights[(r, c)] == 0 {
                let result = bfs(&(r, c), |p| reachable(map, p), |p| *p == map.end);
                if let Some(path) = result {
                    best = usize::min(best, path.len() - 1);
                }
            }
        }
    }
    best
}

fn main() {
    let input = include_str!("../../input/12.txt");
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
    const SAMPLE: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(31, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(29, super::part2(&input));
    }
}
