pub fn part_01() {
    let file_data = include_str!("./input/day_05/input_test.txt");

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let lines: Vec<&str> = file_data.lines().collect();
    let mut y: usize = 0;

    for line in &lines {
        if line.trim().len() == 0 {
            break;
        }

        let chars: Vec<char> = line.chars().collect();

        let mut x: usize = 0;

        for c in chars {
            if c.is_digit(10) {
                let column_number = c
                    .to_digit(10)
                    .expect("Can't convert column number to digit");

                println!("column_number: {}", column_number);

                let mut stack: Vec<char> = Vec::new();
                let mut y_temp: usize = y - 1;

                while y_temp > 0 {
                    let c_temp: char = lines[y_temp].chars().nth(0).unwrap();
                    if c_temp.is_ascii() {
                        stack.push(c_temp);
                    }
                    y_temp = y_temp - 1;
                }
            }

            x = x + 1;
        }
        y = y + 1;
    }

    let result = "no bueno";
    println!("day_05_part_01: {}", result);
}
