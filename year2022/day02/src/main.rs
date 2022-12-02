#[derive(PartialEq, Clone)]
enum rps_enum {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    let input = include_str!("day2input.txt");
    let mut split_stuff = input.lines();

    let mut score: i32 = 0;
    while let Some(string) = split_stuff.next() {
        let split_line: Vec<&str> = string.split_whitespace().collect();
        let enemy_input: rps_enum;
        let your_input: rps_enum;
        match split_line[0] {
            "A" => enemy_input = rps_enum::Rock,
            "B" => enemy_input = rps_enum::Paper,
            "C" => enemy_input = rps_enum::Scissors,
            _ => {
                enemy_input = rps_enum::Rock;
            }
        }
        match split_line[1] {
            "X" => your_input = rps_enum::Rock,
            "Y" => your_input = rps_enum::Paper,
            "Z" => your_input = rps_enum::Scissors,
            _ => {
                your_input = rps_enum::Rock;
            }
        }

        //draw
        if enemy_input == your_input {
            score += your_input as i32 + 3;
        } else {
            match your_input {
                rps_enum::Rock => match enemy_input {
                    rps_enum::Scissors => {
                        score += your_input as i32 + 6;
                    }
                    _ => {
                        score += your_input as i32 + 0;
                    }
                },
                rps_enum::Scissors => match enemy_input {
                    rps_enum::Paper => {
                        score += your_input as i32 + 6;
                    }
                    _ => {
                        score += your_input as i32 + 0;
                    }
                },
                rps_enum::Paper => match enemy_input {
                    rps_enum::Rock => {
                        score += your_input as i32 + 6;
                    }
                    _ => {
                        score += your_input as i32 + 0;
                    }
                },
            }
        }
    }

    println!("Score: {}", score);

    split_stuff = input.lines();

    let mut part2_score = 0;
    while let Some(string) = split_stuff.next() {
        let split_line: Vec<&str> = string.split_whitespace().collect();
        let enemy_input: rps_enum;
        let your_input: rps_enum;
        match split_line[0] {
            "A" => enemy_input = rps_enum::Rock,
            "B" => enemy_input = rps_enum::Paper,
            "C" => enemy_input = rps_enum::Scissors,
            _ => {
                enemy_input = rps_enum::Rock;
            }
        }
        match split_line[1] {
            "Z" => match enemy_input {
                rps_enum::Rock => {
                    your_input = rps_enum::Paper;
                }
                rps_enum::Scissors => {
                    your_input = rps_enum::Rock;
                }
                rps_enum::Paper => {
                    your_input = rps_enum::Scissors;
                }
            },
            "Y" => your_input = enemy_input.clone(),
            "X" => match enemy_input {
                rps_enum::Rock => {
                    your_input = rps_enum::Scissors;
                }
                rps_enum::Scissors => {
                    your_input = rps_enum::Paper;
                }
                rps_enum::Paper => {
                    your_input = rps_enum::Rock;
                }
            },
            _ => {
                your_input = rps_enum::Rock;
            }
        }

        //draw
        if enemy_input == your_input {
            part2_score += your_input as i32 + 3;
        } else {
            match your_input {
                rps_enum::Rock => match enemy_input {
                    rps_enum::Scissors => {
                        part2_score += your_input as i32 + 6;
                    }
                    _ => {
                        part2_score += your_input as i32 + 0;
                    }
                },
                rps_enum::Scissors => match enemy_input {
                    rps_enum::Paper => {
                        part2_score += your_input as i32 + 6;
                    }
                    _ => {
                        part2_score += your_input as i32 + 0;
                    }
                },
                rps_enum::Paper => match enemy_input {
                    rps_enum::Rock => {
                        part2_score += your_input as i32 + 6;
                    }
                    _ => {
                        part2_score += your_input as i32 + 0;
                    }
                },
            }
        }
    }

    println!("part2 score: {}", part2_score);
}
