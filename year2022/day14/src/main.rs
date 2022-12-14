fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 500]; 700];
    let mut bottom = 0;
    while let Some(line) = lines.next() {
        let mut index = 0;
        let number_pairs = line.split(" -> ");
        let mut last_pair: (usize, usize) = (0, 0);
        for number_pair in number_pairs {
            let mut numbers = number_pair.split(",");
            let left_number = numbers.next().unwrap().parse::<usize>().unwrap();
            let right_number = numbers.next().unwrap().parse::<usize>().unwrap();
            if right_number > bottom {
                bottom = right_number;
            }
            if index > 0 {
                if last_pair.0 < left_number {
                    for i in last_pair.0..=left_number {
                        grid[i][last_pair.1] = '#';
                    }
                } else if last_pair.0 > left_number {
                    for i in left_number..=last_pair.0 {
                        grid[i][last_pair.1] = '#';
                    }
                } else if last_pair.1 < right_number {
                    for i in last_pair.1..=right_number {
                        grid[last_pair.0][i] = '#';
                    }
                } else if last_pair.1 > right_number {
                    for i in right_number..=last_pair.1 {
                        grid[last_pair.0][i] = '#';
                    }
                }
            }

            last_pair = (left_number, right_number);
            index += 1;
        }
    }

    //part2
    for i in 0..grid.len() {
        grid[i][bottom + 2] = '#';
    }

    let mut sand_falling_start_pos: (usize, usize) = (500, 0);
    let mut number_of_steps = 0;

    loop {
        let mut current_sand_coordinate: (usize, usize) = sand_falling_start_pos;
        while true {
            let char_below_me = grid[current_sand_coordinate.0][current_sand_coordinate.1 + 1];
            if char_below_me == '.' {
                current_sand_coordinate =
                    (current_sand_coordinate.0, current_sand_coordinate.1 + 1);
            } else if char_below_me == 'O' || char_below_me == '#' {
                let char_left_below_me =
                    grid[current_sand_coordinate.0 - 1][current_sand_coordinate.1 + 1];
                if char_left_below_me == '.' {
                    current_sand_coordinate =
                        (current_sand_coordinate.0 - 1, current_sand_coordinate.1 + 1);
                } else {
                    let char_right_below_me =
                        grid[current_sand_coordinate.0 + 1][current_sand_coordinate.1 + 1];
                    if char_right_below_me == '.' {
                        current_sand_coordinate =
                            (current_sand_coordinate.0 + 1, current_sand_coordinate.1 + 1);
                    } else {
                        if current_sand_coordinate.0 == 500 && current_sand_coordinate.1 == 0 {
                            println!("Steps for part2: {}", number_of_steps + 1);
                            return;
                        }
                        grid[current_sand_coordinate.0][current_sand_coordinate.1] = 'O';
                        break;
                    }
                }
            }
            if current_sand_coordinate.1 > bottom + 3 {
                println!("steps: {}", number_of_steps);

                return;
            }
        }
        number_of_steps += 1;
    }
}
