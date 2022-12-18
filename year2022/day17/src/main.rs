#[derive(Debug, Clone)]
struct Rock {
    current_index: (usize, usize),
    occupied_indexes: Vec<Vec<usize>>, // left edge is 0
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut pushing: Vec<char> = Vec::new();
    while let Some(big_line) = lines.next() {
        for c in big_line.chars() {
            pushing.push(c);
        }
    }
    //[first_index][second_index] : [how_high][how_wide]
    let mut chamber: Vec<Vec<char>> = vec![vec!['.'; 7]; 310000000];

    let mut rocks: Vec<Rock> = Vec::new();
    let rock1 = Rock {
        current_index: (0, 0),
        occupied_indexes: vec![vec![0, 1, 2, 3]; 1],
    };

    rocks.push(rock1);

    let rock2 = Rock {
        current_index: (0, 0),
        occupied_indexes: vec![vec![1], vec![0, 1, 2], vec![1]],
    };
    rocks.push(rock2);

    let rock3 = Rock {
        current_index: (0, 0),
        occupied_indexes: vec![vec![2], vec![2], vec![0, 1, 2]],
    };

    rocks.push(rock3);

    let rock4 = Rock {
        current_index: (0, 0),
        occupied_indexes: vec![vec![0], vec![0], vec![0], vec![0]],
    };

    rocks.push(rock4);

    let rock5 = Rock {
        current_index: (0, 0),
        occupied_indexes: vec![vec![0, 1], vec![0, 1]],
    };

    rocks.push(rock5);

    let mut current_rock_index = 0;
    let mut rocks_fallen: usize = 0;
    let mut steps = 0;
    let mut current_rock: Rock = rocks[current_rock_index].clone();
    let mut highest_occupied_block = 0;
    let mut sum_of_highest_blocks = 0;
    current_rock.current_index = (highest_occupied_block + 3, 2);

    while rocks_fallen < 1000000000000 {
        if rocks_fallen % 1000000 == 0 {
            println!("{} rocks fallen", rocks_fallen);
        }
        if pushing[steps % pushing.len()] == '<' {
            if check_going_left(&chamber, &current_rock) {
                current_rock.current_index.1 -= 1;
            }
        }
        if pushing[steps % pushing.len()] == '>' {
            if check_going_right(&chamber, &current_rock) {
                current_rock.current_index.1 += 1;
            }
        }

        if check_going_down(&chamber, &current_rock) {
            current_rock.current_index.0 -= 1;
        } else {
            for i in 0..current_rock.occupied_indexes.len() {
                for q in 0..current_rock.occupied_indexes[i].len() {
                    let index_to_check = (
                        current_rock.current_index.0 - i,
                        current_rock.current_index.1 + current_rock.occupied_indexes[i][q],
                    );
                    chamber[index_to_check.0][index_to_check.1] = '#';
                }
            }

            for i in 0..current_rock.occupied_indexes.len() {
                let mut full_row = true;
                for q in 0..chamber[current_rock.current_index.0 - i].len() {
                    full_row = full_row && chamber[current_rock.current_index.0 - i][q] == '#';
                    if !full_row {
                        break;
                    }
                }

                if full_row {
                    println!("Im draining");
                    let number_of_drain = i;
                    chamber.drain(0..number_of_drain);
                    highest_occupied_block -= number_of_drain;
                    sum_of_highest_blocks += number_of_drain;

                    for nr in 0..number_of_drain {
                        chamber.push(vec!['.'; 6]);
                    }
                    //shift chamber coordinates, shift highest_occupied_block, add amoutn of rows to sum
                }
            }

            rocks_fallen += 1;
            if current_rock.current_index.0 > highest_occupied_block {
                highest_occupied_block = current_rock.current_index.0;
            }
            current_rock_index += 1;
            current_rock_index = current_rock_index % rocks.len();
            current_rock = rocks[current_rock_index].clone();
            current_rock.current_index = (
                highest_occupied_block + 3 + current_rock.occupied_indexes.len(),
                2,
            );
        }
        steps += 1;
    }
    println!("rocks fallen: {}", rocks_fallen);
    println!("highest occupied block is: {}", highest_occupied_block + 1);
}

fn check_going_left(grid: &Vec<Vec<char>>, rock: &Rock) -> bool {
    let mut can_go_left = true;
    for i in 0..rock.occupied_indexes.len() {
        for q in 0..rock.occupied_indexes[i].len() {
            if rock.current_index.1 + rock.occupied_indexes[i][q] == 0 {
                return false;
            }
            let index_to_check = (
                rock.current_index.0 - i,
                rock.current_index.1 + rock.occupied_indexes[i][q] - 1,
            );
            if grid[index_to_check.0][index_to_check.1] != '.' {
                return false;
            }
        }
    }

    return can_go_left;
}

fn check_going_right(grid: &Vec<Vec<char>>, rock: &Rock) -> bool {
    let mut can_go_left = true;

    for i in 0..rock.occupied_indexes.len() {
        for q in 0..rock.occupied_indexes[i].len() {
            if rock.current_index.1 + rock.occupied_indexes[i][q] >= grid[0].len() - 1 {
                return false;
            }
            let index_to_check = (
                rock.current_index.0 - i,
                rock.current_index.1 + rock.occupied_indexes[i][q] + 1,
            );
            if grid[index_to_check.0][index_to_check.1] != '.' {
                return false;
            }
        }
    }

    return can_go_left;
}

fn check_going_down(grid: &Vec<Vec<char>>, rock: &Rock) -> bool {
    let can_go_down = true;
    for i in 0..rock.occupied_indexes.len() {
        for q in 0..rock.occupied_indexes[i].len() {
            if rock.current_index.0 < rock.occupied_indexes.len() {
                return false;
            }
            let index_to_check = (
                rock.current_index.0 - i - 1,
                rock.current_index.1 + rock.occupied_indexes[i][q],
            );
            if grid[index_to_check.0][index_to_check.1] != '.' {
                return false;
            }
        }
    }
    return can_go_down;
}
