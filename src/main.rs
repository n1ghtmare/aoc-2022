use std::fs;

// Day 1
fn main() {
    let file_data: String =
        fs::read_to_string("input_prod.txt").expect("Can't read the contents of the file.");

    let mut calories: Vec<usize> = file_data
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .flat_map(|y| usize::from_str_radix(y, 10))
                .sum::<usize>()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    let sum: usize = calories.iter().take(3).sum();

    println!("Result: {}", sum);
}