use std::collections::HashMap;

fn main() {
    // day_1();

    day_2_part_2();
}

/*
Rules:
---

Scores:
-
Rock: 1
Paper: 2
Scissors: 3

Outcomes:
-
Loss: 0
Draw: 3
Win: 6

*/

#[allow(dead_code)]
fn day_2_part_1() {
    let shapes: HashMap<&str, u32> =
        HashMap::from([("A", 1), ("X", 1), ("B", 2), ("Y", 2), ("C", 3), ("Z", 3)]);

    let mut score: u32 = 0;

    let file_data = include_str!("./input/day2/input_prod.txt");

    for line in file_data.lines() {
        let round_shapes = line.split(" ").collect::<Vec<&str>>();

        let o = shapes.get(round_shapes[0]).unwrap().clone();
        let p = shapes.get(round_shapes[1]).unwrap().clone();

        let round_outcome: u32 = if (p == 3 && o == 2) || (p == 2 && o == 1) || (p == 1 && o == 3) {
            6
        } else if p == o {
            3
        } else {
            0
        };

        score = score + round_outcome + p;
    }

    println!("score: {}", score);

    // dbg!("{}", lines);
}

/*
Scores:
-
Rock: 1
Paper: 2
Scissors: 3

X: Loose = 0
Y: Draw = 3
Z: Win = 6
*/
fn day_2_part_2() {
    let shapes: HashMap<&str, u32> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let mut score: u32 = 0;

    let file_data = include_str!("./input/day2/input_prod.txt");

    for line in file_data.lines() {
        let columns = line.split(" ").collect::<Vec<&str>>();

        let o = shapes.get(columns[0]).unwrap().clone();

        let round_outcome = match columns[1] {
            "X" => match o {
                3 => 2,
                2 => 1,
                1 => 3,
                _ => 0,
            },
            "Y" => 3 + o,
            "Z" => {
                6 + match o {
                    3 => 1,
                    2 => 3,
                    1 => 2,
                    _ => 0,
                }
            }
            _ => 0,
        };

        score = score + round_outcome;
    }

    println!("score: {}", score);

    // dbg!("{}", lines);
}

#[allow(dead_code)]
fn day_1() {
    let file_data = include_str!("./input/day1/input_prod.txt");

    let mut calories: Vec<u32> = file_data
        .split("\n\n")
        .map(|x| x.lines().flat_map(|y| y.parse::<u32>()).sum())
        .collect();

    calories.sort_unstable_by(|a, b| b.cmp(a));

    let sum: u32 = calories.iter().take(3).sum();

    println!("Result: {}", sum);
}
