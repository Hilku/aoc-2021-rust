fn main() {
    let input = include_str!("day11input.txt");
    let mut parts = input.lines();
    let mut octopuses: Vec<Vec<i16>> = vec![Vec::new(); 10];
    let mut current_read_index = 0;

    while let Some(line) = parts.next() {
        for c in line.chars() {
            octopuses[current_read_index].push(c.to_digit(10).unwrap() as i16)
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
                        for i in -1..=1 {
                            for q in -1..=1 {
                                let row_index = row as i32 + i;
                                let column_index = column as i32 + q;

                                if !flashed_octopy
                                    .contains(&(row_index as usize, column_index as usize))
                                {
                                    if let Some(v) = octopouses_clone.get_mut(row_index as usize) {
                                        if let Some(j) = v.get_mut(column_index as usize) {
                                            *j += 1;
                                        }
                                    }
                                }
                            }
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
