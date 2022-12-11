use std::collections::HashSet;

// We start at these indices and we map accordingly
const OFFSET_INDEX_LOWER: u8 = b'a' - 1; // 95
const OFFSET_INDEX_UPPER: u8 = b'A' - 1 - 26; // 64 - 26 (which is the length of the lowecase scores)

fn get_sum_of_char_set(set: HashSet<char>) -> u32 {
    set.iter()
        .map(|c| {
            let result = if c.is_ascii_lowercase() {
                c.clone() as u8 - OFFSET_INDEX_LOWER
            } else {
                c.clone() as u8 - OFFSET_INDEX_UPPER
            };
            result as u32
        })
        .sum()
}

pub fn part_01() {
    let file_data = include_str!("./input/day_03/input_prod.txt");

    let sum: u32 = file_data
        .lines()
        .map(|x| {
            let (left, right) = x.split_at(x.len() / 2);

            let left = left.chars().collect::<HashSet<char>>();
            let right = right.chars().collect::<HashSet<char>>();

            let set: HashSet<char> = left.intersection(&right).map(|c| c.clone()).collect();
            get_sum_of_char_set(set)
        })
        .sum();

    println!("day_03_part_01: {}", sum);
}

pub fn part_02() {
    let file_data = include_str!("./input/day_03/input_prod.txt");

    let sum: u32 = file_data
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let char_sets: Vec<HashSet<char>> = chunk
                .iter()
                .map(|x| x.chars().collect::<HashSet<char>>())
                .collect();

            let mut char_sets_intersections: HashSet<char> = char_sets.first().unwrap().clone();

            for set in &char_sets[1..] {
                char_sets_intersections = char_sets_intersections
                    .intersection(&set)
                    .map(|c| c.clone())
                    .collect::<HashSet<char>>();
            }

            get_sum_of_char_set(char_sets_intersections)
        })
        .sum();

    println!("day_03_part_02: {}", sum);
}
