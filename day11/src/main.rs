fn main() {
    let input = include_str!("day11input.txt");
    let mut parts = input.lines();

    let mut octopuses: Vec<Vec<i16>> = vec![vec![-1; 10]; 10];

    let mut current_read_index = 0;

    while let Some(line) = parts.next() {
        let mut second_index = 0;
        for c in line.chars() {
            octopuses[current_read_index][second_index] = c.to_digit(10).unwrap() as i16;
            second_index += 1;
        }
        current_read_index += 1;
    }

    let mut octopouses_clone = octopuses.clone();
    let mut flashes: usize = 0;

    let mut day: usize = 0;

    let mut flashed_octopy: Vec<(usize, usize)> = Vec::new();
    while flashed_octopy.len() < 100 {
        flashed_octopy.clear();
        for row in 0..octopouses_clone.len() {
            for column in 0..octopouses_clone[row].len() {
                octopouses_clone[row][column] += 1;
            }
        }
        let mut flashed: bool = true;
        while flashed {
            flashed = false;
            for row in 0..octopouses_clone.len() {
                for column in 0..octopouses_clone[row].len() {
                    if octopouses_clone[row][column] > 9 && !flashed_octopy.contains(&(row, column))
                    {
                        flashed_octopy.push((row, column));
                        flashed = true;
                        flashes += 1;
                        if row > 0 {
                            if !flashed_octopy.contains(&(row - 1, column)) {
                                octopouses_clone[row - 1][column] += 1;
                            }
                            if column > 0 && !flashed_octopy.contains(&(row - 1, column - 1)) {
                                octopouses_clone[row - 1][column - 1] += 1;
                            }
                            if column < octopouses_clone[row].len() - 1
                                && !flashed_octopy.contains(&(row - 1, column + 1))
                            {
                                octopouses_clone[row - 1][column + 1] += 1;
                            }
                        }
                        if row < octopouses_clone.len() - 1 {
                            if !flashed_octopy.contains(&(row + 1, column)) {
                                octopouses_clone[row + 1][column] += 1;
                            }
                            if column < octopouses_clone[row].len() - 1
                                && !flashed_octopy.contains(&(row + 1, column + 1))
                            {
                                octopouses_clone[row + 1][column + 1] += 1;
                            }
                            if column > 0 && !flashed_octopy.contains(&(row + 1, column - 1)) {
                                octopouses_clone[row + 1][column - 1] += 1
                            };
                        }
                        if column < octopouses_clone[row].len() - 1
                            && !flashed_octopy.contains(&(row, column + 1))
                        {
                            octopouses_clone[row][column + 1] += 1;
                        }
                        if column > 0 && !flashed_octopy.contains(&(row, column - 1)) {
                            octopouses_clone[row][column - 1] += 1;
                        }
                        octopouses_clone[row][column] = 0;
                    }
                }
            }
        }
        if day == 99 {
            println!("Flashes: {}", flashes);
        }
        day += 1;
    }

    println!("Day: {}", day);
}
