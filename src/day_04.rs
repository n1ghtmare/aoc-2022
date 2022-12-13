/*
Sample input:
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
*/

use std::str::FromStr;

struct Assignment {
    start: u8,
    end: u8,
}

// TODO: Read a bit on anyhow?
// TODO: Read about error handling in rust (what's an idiomatic way to handle errors)
impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once("-")
            .map(|(left, right)| (left.parse::<u8>().unwrap(), right.parse::<u8>().unwrap()))
            .unwrap();

        Ok(Assignment { start, end })
    }
}

pub fn part_01() {
    let file_data = include_str!("./input/day_04/input_prod.txt");

    let sum: u32 = file_data
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(",")
                .map(|(l, r)| {
                    (
                        Assignment::from_str(l).unwrap(),
                        Assignment::from_str(r).unwrap(),
                    )
                })
                .unwrap();

            if left.start <= right.start && left.end >= right.end {
                1
            } else if left.start >= right.start && left.end <= right.end {
                1
            } else {
                0
            }
        })
        .sum();

    println!("day_04_part_01: {}", sum);
}

fn is_in_between(n: u8, start: u8, end: u8) -> bool {
    n >= start && n <= end
}

pub fn part_02() {
    let file_data = include_str!("./input/day_04/input_prod.txt");

    let sum: u32 = file_data
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(",")
                .map(|(l, r)| {
                    (
                        Assignment::from_str(l).unwrap(),
                        Assignment::from_str(r).unwrap(),
                    )
                })
                .unwrap();

            if is_in_between(left.start, right.start, right.end)
                || is_in_between(left.end, right.start, right.end)
            {
                1
            } else if is_in_between(right.start, left.start, left.end)
                || is_in_between(right.end, left.start, left.end)
            {
                1
            } else {
                0
            }
        })
        .sum();

    println!("day_04_part_02: {}", sum);
}
