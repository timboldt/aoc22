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

extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

use std::time::Instant;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Polyline {
    points: Vec<Point>,
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    let (i, (x, y)) = separated_pair(
        map_res(digit1, |s: &str| s.parse::<i32>()),
        char(','),
        map_res(digit1, |s: &str| s.parse::<i32>()),
    )(i)?;
    Ok((i, Point { x, y }))
}

fn parse_polyline(i: &str) -> IResult<&str, Polyline> {
    let (i, points) = separated_list1(tag(" -> "), parse_point)(i)?;
    Ok((i, Polyline { points }))
}

fn parse(input: &str) -> Vec<Polyline> {
    for line in input.lines() {
        let (_, p) = parse_polyline(line).unwrap();
        println!("{:?}", p);
    }
    //let mut parser = recognize(separated_pair(digit1, char(','), digit1));
    // let (a, b) = parser("123,456").unwrap();
    // println!("{},{}", a, b);
    // for line in input.lines() {}
    vec![]
}

fn in_correct_order(left: &str, right: &str) -> bool {
    // TODO: Implement me.
    true
}

fn part1(parsed: &[Polyline]) -> usize {
    42
}

fn part2(parsed: &[Polyline]) -> usize {
    42
}

fn main() {
    let input = include_str!("../../input/14.txt");
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
    const SAMPLE: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(13, super::part1(&input));
    }

    #[test]
    fn part2_works() {
        let input = super::parse(SAMPLE);
        assert_eq!(29, super::part2(&input));
    }
}
