#[derive(Clone, PartialEq, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut head_coordinate: Coordinate = Coordinate { x: 0, y: 0 };

    let mut tail_coordinate: Coordinate = Coordinate { x: 0, y: 0 };

    let mut tail_positions: Vec<Coordinate> = Vec::new();
    tail_positions.push(tail_coordinate.clone());

    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let number_of_steps = parts[1].parse::<i64>().unwrap();

        if parts[0] == "R" {
            for i in 0..number_of_steps {
                head_coordinate.x += 1;
                if head_coordinate.x - tail_coordinate.x.clone() > 1 {
                    tail_coordinate.x += 1;
                    if tail_coordinate.y < head_coordinate.y {
                        tail_coordinate.y += 1;
                    } else if tail_coordinate.y > head_coordinate.y {
                        tail_coordinate.y -= 1;
                    }
                    if !tail_positions.contains(&tail_coordinate) {
                        tail_positions.push(tail_coordinate.clone());
                    }
                }
            }
        }

        if parts[0] == "L" {
            for i in 0..number_of_steps {
                head_coordinate.x -= 1;
                if tail_coordinate.x.clone() - head_coordinate.x > 1 {
                    tail_coordinate.x -= 1;
                    if tail_coordinate.y < head_coordinate.y {
                        tail_coordinate.y += 1;
                    } else if tail_coordinate.y > head_coordinate.y {
                        tail_coordinate.y -= 1;
                    }
                    if !tail_positions.contains(&tail_coordinate) {
                        tail_positions.push(tail_coordinate.clone());
                    }
                }
            }
        }

        if parts[0] == "U" {
            for i in 0..number_of_steps {
                head_coordinate.y += 1;
                if head_coordinate.y - tail_coordinate.y.clone() > 1 {
                    tail_coordinate.y += 1;
                    if tail_coordinate.x < head_coordinate.x {
                        tail_coordinate.x += 1;
                    } else if tail_coordinate.x > head_coordinate.x {
                        tail_coordinate.x -= 1;
                    }
                    if !tail_positions.contains(&tail_coordinate) {
                        tail_positions.push(tail_coordinate.clone());
                    }
                }
            }
        }

        if parts[0] == "D" {
            for i in 0..number_of_steps {
                head_coordinate.y -= 1;
                if tail_coordinate.y.clone() - head_coordinate.y > 1 {
                    tail_coordinate.y -= 1;
                    if tail_coordinate.x < head_coordinate.x {
                        tail_coordinate.x += 1;
                    } else if tail_coordinate.x > head_coordinate.x {
                        tail_coordinate.x -= 1;
                    }
                    if !tail_positions.contains(&tail_coordinate) {
                        tail_positions.push(tail_coordinate.clone());
                    }
                }
            }
        }
    }
    //0 head, 9: tail_needed
    let mut tail_coordinates: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }; 10];
    let mut part2_positions: Vec<Coordinate> = Vec::new();
    part2_positions.push(Coordinate { x: 0, y: 0 });

    let mut lines2 = input.lines();
    while let Some(line) = lines2.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let number_of_steps = parts[1].parse::<i64>().unwrap();

        let dir = parts[0];
        for i in 0..number_of_steps {
            if dir == "R" {
                tail_coordinates[0].x += 1;
            } else if dir == "L" {
                tail_coordinates[0].x -= 1;
            } else if dir == "U" {
                tail_coordinates[0].y += 1;
            } else if dir == "D" {
                tail_coordinates[0].y -= 1;
            }

            for i in 1..tail_coordinates.len() {
                let cor: Coordinate = move_tail(
                    tail_coordinates[i - 1].clone(),
                    tail_coordinates[i].clone(),
                    dir.to_string(),
                );
                tail_coordinates[i] = cor.clone();
                if i == tail_coordinates.len() - 1 {
                    if !part2_positions.contains(&tail_coordinates[i]) {
                        println!("new position for endtail: {:?}", tail_coordinates[i]);
                        part2_positions.push(tail_coordinates[i].clone());
                    }
                }
            }
        }

        println!("command: {}, tail pos: {:?}", line, tail_coordinates[6]);
    }

    println!("Coordinates for tail1: {}", tail_positions.len());

    println!("Coordinates for tail9: {}", part2_positions.len());
}

fn move_tail(tail_before: Coordinate, tail_to_move: Coordinate, direction: String) -> Coordinate {
    let mut end_result = tail_to_move.clone();
    if direction == "R" {
        if tail_before.x - tail_to_move.x > 1 {
            end_result.x += 1;

            if tail_to_move.y < tail_before.y {
                end_result.y += 1;
            } else if tail_to_move.y > tail_before.y {
                end_result.y -= 1;
            }
        }
    }

    if direction == "L" {
        if tail_to_move.x - tail_before.x > 1 {
            end_result.x -= 1;

            if tail_to_move.y < tail_before.y {
                end_result.y += 1;
            } else if tail_to_move.y > tail_before.y {
                end_result.y -= 1;
            }
        }
    }

    if direction == "U" {
        if tail_before.y - tail_to_move.y > 1 {
            end_result.y += 1;

            if tail_to_move.x < tail_before.x {
                end_result.x += 1;
            } else if tail_to_move.x > tail_before.x {
                end_result.x -= 1;
            }
        }
    }

    if direction == "D" {
        if tail_to_move.y - tail_before.y > 1 {
            end_result.y -= 1;

            if tail_to_move.x < tail_before.x {
                end_result.x += 1;
            } else if tail_to_move.x > tail_before.x {
                end_result.x -= 1;
            }
        }
    }

    return end_result;
}
