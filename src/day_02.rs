/*
Sign Scores
----
             O   P
             _____
Rock        (A | X): 1
Paper       (B | Y): 2
Scissors    (C | Z): 3

Outcomes:
----
Loss: 0
Draw: 3
Win: 6

Winning combinations (for P):
----
CX, 1
AY, 2
BZ, 3

Losing combinations (for P):
----
AZ, 3
BX, 1
CY, 2

Draw combinations:
----
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

fn get_score(sign: Sign, outcome: Outcome) -> u8 {
    let sign: u8 = match sign {
        Sign::Rock => 1,
        Sign::Paper => 2,
        Sign::Scissors => 3,
    };

    let outcome: u8 = match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    };

    sign + outcome
}

fn get_score_by_combination(combination: &str) -> Option<u8> {
    match combination {
        "CX" => Some(get_score(Sign::Rock, Outcome::Win)),
        "AY" => Some(get_score(Sign::Paper, Outcome::Win)),
        "BZ" => Some(get_score(Sign::Scissors, Outcome::Win)),
        "AZ" => Some(get_score(Sign::Scissors, Outcome::Loss)),
        "BX" => Some(get_score(Sign::Rock, Outcome::Loss)),
        "CY" => Some(get_score(Sign::Paper, Outcome::Loss)),
        "AX" => Some(get_score(Sign::Rock, Outcome::Draw)),
        "BY" => Some(get_score(Sign::Paper, Outcome::Draw)),
        "CZ" => Some(get_score(Sign::Scissors, Outcome::Draw)),
        _ => None,
    }
}
pub fn part_01() {
    let file_data = include_str!("./input/day_02/input_prod.txt");
    let score: u32 = file_data
        .lines()
        .map(
            |x| match get_score_by_combination(x.replace(" ", "").as_str()) {
                Some(o) => o as u32,
                None => 0,
            },
        )
        .sum();

    println!("day_02_part_01: {}", score);
}

/*
Scores:
----
Rock: 1
Paper: 2
Scissors: 3

X: Loose = 0
Y: Draw = 3
Z: Win = 6
*/
fn get_score_for_round(expected_outcome: &str, opponent_shape: &str) -> u32 {
    match expected_outcome {
        // Loss
        "X" => match opponent_shape {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => 0,
        },
        // Win
        "Z" => {
            6 + match opponent_shape {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0,
            }
        }
        // Draw
        "Y" => {
            3 + match opponent_shape {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}

pub fn part_02() {
    let file_data = include_str!("./input/day_02/input_prod.txt");

    let score: u32 = file_data
        .lines()
        .map(|x| {
            let (opponent_shape, expected_outcome) = x.split_once(" ").unwrap();
            get_score_for_round(expected_outcome, opponent_shape)
        })
        .sum();

    println!("day_02_part_02: {}", score);
}
