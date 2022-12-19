use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut pushing: Vec<Vec<Vec<char>>> = vec![vec![vec!['.'; 24]; 24]; 24];
    let mut biggest_coordinates: (usize, usize, usize) = (0, 0, 0);
    let mut smallest_coordinates: (usize, usize, usize) = (10000, 10000, 10000);
    while let Some(big_line) = lines.next() {
        let coordinates: Vec<&str> = big_line.split(',').collect();
        let x = coordinates[0].parse::<usize>().unwrap() + 1;
        let y = coordinates[1].parse::<usize>().unwrap() + 1;
        let z = coordinates[2].parse::<usize>().unwrap() + 1;
        if x < smallest_coordinates.0 {
            smallest_coordinates.0 = x;
        }
        if x > biggest_coordinates.0 {
            biggest_coordinates.0 = x;
        }
        if y < smallest_coordinates.1 {
            smallest_coordinates.1 = y;
        }
        if y > biggest_coordinates.1 {
            biggest_coordinates.1 = y;
        }
        if z < smallest_coordinates.2 {
            smallest_coordinates.2 = z;
        }
        if z > biggest_coordinates.2 {
            biggest_coordinates.2 = z;
        }

        pushing[x][y][z] = '#';
    }

    let mut sum = 0;
    for x_coordinate in 0..pushing.len() {
        for y_coordinate in 0..pushing[x_coordinate].len() {
            for z_coordinate in 0..pushing[x_coordinate][y_coordinate].len() {
                if pushing[x_coordinate][y_coordinate][z_coordinate] == '#' {
                    let mut open_sides = 0;
                    if pushing[x_coordinate - 1][y_coordinate][z_coordinate] == '.' {
                        open_sides += 1;
                    }
                    if pushing[x_coordinate + 1][y_coordinate][z_coordinate] == '.' {
                        open_sides += 1;
                    }
                    if pushing[x_coordinate][y_coordinate - 1][z_coordinate] == '.' {
                        open_sides += 1;
                    }
                    if pushing[x_coordinate][y_coordinate + 1][z_coordinate] == '.' {
                        open_sides += 1;
                    }
                    if pushing[x_coordinate][y_coordinate][z_coordinate - 1] == '.' {
                        open_sides += 1;
                    }
                    if pushing[x_coordinate][y_coordinate][z_coordinate + 1] == '.' {
                        open_sides += 1;
                    }
                    sum += open_sides;
                }
            }
        }
    }

    let mut coordinates_to_check: VecDeque<(usize, usize, usize)> = VecDeque::new();
    coordinates_to_check.push_back((0, 0, 0));
    let mut already_checked_coordinates: Vec<(usize, usize, usize)> = vec![];
    let mut outside_sum = 0;
    while coordinates_to_check.len() > 0 {
        let coordinate = coordinates_to_check.pop_front().unwrap();
        if already_checked_coordinates.contains(&coordinate) {
            continue;
        }
        already_checked_coordinates.push(coordinate.clone());
        let x_coordinate = coordinate.0;
        let y_coordinate = coordinate.1;
        let z_coordinate = coordinate.2;
        if x_coordinate > 0 {
            if pushing[x_coordinate - 1][y_coordinate][z_coordinate] == '#' {
                outside_sum += 1;
            } else if !already_checked_coordinates.contains(&(
                x_coordinate - 1,
                y_coordinate,
                z_coordinate,
            )) {
                coordinates_to_check.push_back((x_coordinate - 1, y_coordinate, z_coordinate));
            }
        }
        if x_coordinate < pushing.len() - 1 {
            if pushing[x_coordinate + 1][y_coordinate][z_coordinate] == '#' {
                outside_sum += 1;
            } else if !already_checked_coordinates.contains(&(
                x_coordinate + 1,
                y_coordinate,
                z_coordinate,
            )) {
                coordinates_to_check.push_back((x_coordinate + 1, y_coordinate, z_coordinate));
            }
        }
        if y_coordinate > 0 {
            if pushing[x_coordinate][y_coordinate - 1][z_coordinate] == '#' {
                outside_sum += 1;
            } else if !already_checked_coordinates.contains(&(
                x_coordinate,
                y_coordinate - 1,
                z_coordinate,
            )) {
                coordinates_to_check.push_back((x_coordinate, y_coordinate - 1, z_coordinate));
            }
        }
        if y_coordinate < pushing.len() - 1 {
            if pushing[x_coordinate][y_coordinate + 1][z_coordinate] == '#' {
                outside_sum += 1;
            } else if !already_checked_coordinates.contains(&(
                x_coordinate,
                y_coordinate + 1,
                z_coordinate,
            )) {
                coordinates_to_check.push_back((x_coordinate, y_coordinate + 1, z_coordinate));
            }
        }
        if z_coordinate > 0 {
            if pushing[x_coordinate][y_coordinate][z_coordinate - 1] == '#' {
                outside_sum += 1;
            } else if !already_checked_coordinates.contains(&(
                x_coordinate,
                y_coordinate,
                z_coordinate - 1,
            )) {
                coordinates_to_check.push_back((x_coordinate, y_coordinate, z_coordinate - 1));
            }
        }
        if z_coordinate < pushing.len() - 1 {
            if pushing[x_coordinate][y_coordinate][z_coordinate + 1] == '#' {
                outside_sum += 1;
            } else if !already_checked_coordinates.contains(&(
                x_coordinate,
                y_coordinate,
                z_coordinate + 1,
            )) {
                coordinates_to_check.push_back((x_coordinate, y_coordinate, z_coordinate + 1));
            }
        }
    }

    println!("solution: {}", sum);
    println!("solutoin part2: {}", outside_sum);
}
