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

use std::{collections::HashMap, time::Instant};

fn propagate(cwd: &Vec<String>, cwd_size: usize, dir_sizes: &mut HashMap<String, usize>) {
    let mut key: String = "/".into();
    if dir_sizes.contains_key(&key) {
        *dir_sizes.get_mut(&key).unwrap() += cwd_size;
    } else {
        dir_sizes.insert(key.clone(), cwd_size);
    }
    for part in cwd {
        key = key + "/" + &part;
        if dir_sizes.contains_key(&key) {
            *dir_sizes.get_mut(&key).unwrap() += cwd_size;
        } else {
            dir_sizes.insert(key.clone(), cwd_size);
        }
    }
}

fn parse(input: &str) -> HashMap<String, usize> {
    let mut dir_sizes = HashMap::<String, usize>::new();
    let mut cwd: Vec<String> = vec![];
    let mut cwd_size: usize = 0;
    for line in input.lines() {
        match line.chars().nth(0).unwrap() {
            '$' => {
                propagate(&cwd, cwd_size, &mut dir_sizes);
                cwd_size = 0;
                if line.starts_with("$ cd") {
                    let dir = line.split_whitespace().last().unwrap();
                    if dir == "/" {
                        cwd.clear();
                    } else if dir == ".." {
                        cwd.pop().unwrap();
                    } else {
                        cwd.push(dir.to_owned());
                    }
                } else if line.starts_with("$ ls") {
                    // Nothing to do.
                } else {
                    unreachable!();
                }
            }
            'd' => {}
            '0'..='9' => {
                cwd_size += line
                    .split_whitespace()
                    .nth(0)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            _ => unreachable!(),
        }
    }
    propagate(&cwd, cwd_size, &mut dir_sizes);
    dir_sizes
}

fn part1(puzzle: &HashMap<String, usize>) -> usize {
    puzzle.values().filter(|v| **v <= 100000).sum()
}

fn part2(puzzle: &HashMap<String, usize>) -> usize {
    const FS_SIZE: usize = 70000000;
    const UPGD_SIZE: usize = 30000000;
    let fs_used = puzzle.get("/").unwrap();
    let space_needed = UPGD_SIZE - (FS_SIZE - fs_used);
    *puzzle
        .values()
        .filter(|v| **v >= space_needed)
        .reduce(|accum, v| if v < accum { v } else { accum })
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/07.txt");
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
    const SAMPLE: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        println!("{:?}", input);
        assert_eq!(95437, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(24933642, super::part2(&input));
    }
}
