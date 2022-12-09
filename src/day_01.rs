pub fn part_01() {
    let file_data = include_str!("./input/day_01/input_prod.txt");

    let sum: u32 = file_data
        .split("\n\n")
        .map(|x| x.lines().flat_map(|y| y.parse::<u32>()).sum::<u32>())
        .max()
        .unwrap();

    println!("day_01_part_01: {}", sum);
}

pub fn part_02() {
    let file_data = include_str!("./input/day_01/input_prod.txt");

    let mut calories: Vec<u32> = file_data
        .split("\n\n")
        .map(|x| x.lines().flat_map(|y| y.parse::<u32>()).sum())
        .collect();

    calories.sort_unstable_by(|a, b| b.cmp(a));

    let sum: u32 = calories.iter().take(3).sum();

    println!("day_01_part_02: {}", sum);
}
