use std::collections::HashMap;

fn main() {
    day_2_part_1();
}

/*
Rules:
---

Scores:
-
             O   P
             _____
Rock        (A | X): 1
Paper       (B | Y): 2
Scissors    (C | Z): 3

Outcomes:
-
Loss: 0
Draw: 3
Win: 6

Winning combinations (for P):
CX, 1
AY, 2
BZ, 3

Losing combinations (for P):
AZ, 3
BX, 1
CY, 2

Draw combinations:
AX, 1
BY, 2
CZ, 3
*/

enum Outcome {
    Win,
    Loss,
    Draw,
}

enum Sign {
    Rock,
    Paper,
    Scissors,
}

fn get_score(sign: Sign, outcome: Outcome) -> usize {
    let sign: usize = match sign {
        Sign::Rock => 1,
        Sign::Paper => 2,
        Sign::Scissors => 3,
    };

    let outcome: usize = match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    };

    sign + outcome
}

fn day_2_part_1() {
    let outcome_combinations: HashMap<&str, usize> = HashMap::from([
        ("CX", get_score(Sign::Rock, Outcome::Win)),
        ("AY", get_score(Sign::Paper, Outcome::Win)),
        ("BZ", get_score(Sign::Scissors, Outcome::Win)),
        ("AZ", get_score(Sign::Scissors, Outcome::Loss)),
        ("BX", get_score(Sign::Rock, Outcome::Loss)),
        ("CY", get_score(Sign::Paper, Outcome::Loss)),
        ("AX", get_score(Sign::Rock, Outcome::Draw)),
        ("BY", get_score(Sign::Paper, Outcome::Draw)),
        ("CZ", get_score(Sign::Scissors, Outcome::Draw)),
    ]);

    let file_data = include_str!("./input/day2/input_prod.txt");

    let score: usize = file_data
        .lines()
        .map(
            |x| match outcome_combinations.get(x.replace(" ", "").as_str()) {
                Some(o) => o.clone(),
                None => 0,
            },
        )
        .sum();

    println!("score: {}", score);
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

#[allow(dead_code)]
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
