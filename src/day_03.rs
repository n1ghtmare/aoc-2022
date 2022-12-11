use std::collections::HashSet;

// We start at these indices and we map accordingly
const OFFSET_INDEX_LOWER: u8 = b'a' - 1; // 95
const OFFSET_INDEX_UPPER: u8 = b'A' - 1 - 26; // 64 - 26 (which is the length of the lowecase scores)

pub fn part_01() {
    let file_data = include_str!("./input/day_03/input_prod.txt");

    let sum = file_data
        .lines()
        .map(|x| {
            let (left, right) = x.split_at(x.len() / 2);

            let left = left.chars().collect::<HashSet<char>>();
            let right = right.chars().collect::<HashSet<char>>();

            left.intersection(&right)
                .map(|c| {
                    (if c.is_ascii_lowercase() {
                        c.clone() as u8 - OFFSET_INDEX_LOWER
                    } else {
                        c.clone() as u8 - OFFSET_INDEX_UPPER
                    }) as u32
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("day_03_part_01: {:?}", sum);
}
