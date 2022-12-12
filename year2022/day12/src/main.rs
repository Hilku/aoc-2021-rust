#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut line_index = 0;
    //Grid is indexed like this: Y, X
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);
    while let Some(line) = lines.next() {
        let mut line_vec: Vec<char> = Vec::new();
        let mut column: usize = 0;
        for c in line.chars() {
            if c == 'S' {
                start_pos = (line_index, column);
                line_vec.push('a');
                continue;
            }
            if c == 'E' {
                end_pos = (line_index, column);
                line_vec.push('z');
                continue;
            }

            column += 1;
            line_vec.push(c);
        }
        grid.push(line_vec.clone());
        line_index += 1;
    }
    let mut part2_soltuions: Vec<usize> = Vec::new();
    println!("{:?}", grid);
    let mut coordinate_to_steps_map: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..grid.len() {
        for q in 0..grid[i].len() {
            if grid[i][q] == 'a' {
                let mut current_pos = (i, q);
                println!("start_pos: {:?}, end_pos: {:?}", current_pos, end_pos);
                let mut solutions: Vec<usize> = Vec::new();
                start_path(
                    &grid,
                    current_pos,
                    end_pos,
                    0,
                    Vec::new(),
                    &mut solutions,
                    &mut coordinate_to_steps_map,
                );

                if (solutions.len() > 0) {
                    solutions.sort();
                    part2_soltuions.push(solutions[0]);
                }
            }
        }
    }
    part2_soltuions.sort();
    println!("part2 solution: {}", part2_soltuions[0]);
}

fn start_path(
    grid: &Vec<Vec<char>>,
    current_pos: (usize, usize),
    end_pos: (usize, usize),
    number_of_steps: usize,
    mut coordinates_ive_been_to: Vec<(usize, usize)>,
    mut solutions: &mut Vec<usize>,
    mut coordinate_to_steps_map: &mut HashMap<(usize, usize), usize>,
) -> (bool, usize) {
    if coordinate_to_steps_map.contains_key(&current_pos) {
        if coordinate_to_steps_map.get(&current_pos).unwrap() > &number_of_steps {
            *coordinate_to_steps_map.get_mut(&current_pos).unwrap() = number_of_steps;
        } else {
            return (false, 10000);
        }
    } else {
        coordinate_to_steps_map.insert(current_pos, number_of_steps);
    }

    if current_pos == end_pos {
        solutions.push(number_of_steps);
        return (true, number_of_steps);
    }

    if number_of_steps > 468 || coordinates_ive_been_to.contains(&current_pos) {
        return (false, 0);
    }

    coordinates_ive_been_to.push(current_pos);

    let mut solution: (bool, usize) = (false, 10000);
    if can_go_in_direction(grid, current_pos, Direction::Up) {
        let solution_1 = start_path(
            grid,
            (current_pos.0 - 1, current_pos.1),
            end_pos,
            number_of_steps + 1,
            coordinates_ive_been_to.clone(),
            &mut solutions,
            &mut coordinate_to_steps_map,
        );
        if solution_1.0 {
            if solution_1.1 < solution.1 {
                solution.1 = solution_1.1;
            }
        }
    }

    if can_go_in_direction(grid, current_pos, Direction::Down) {
        let solution_1 = start_path(
            grid,
            (current_pos.0 + 1, current_pos.1),
            end_pos,
            number_of_steps + 1,
            coordinates_ive_been_to.clone(),
            &mut solutions,
            &mut coordinate_to_steps_map,
        );
        if solution_1.0 {
            if solution_1.1 < solution.1 {
                solution.1 = solution_1.1;
            }
        }
    }
    if can_go_in_direction(grid, current_pos, Direction::Right) {
        let solution_1 = start_path(
            grid,
            (current_pos.0, current_pos.1 + 1),
            end_pos,
            number_of_steps + 1,
            coordinates_ive_been_to.clone(),
            &mut solutions,
            &mut coordinate_to_steps_map,
        );
        if solution_1.0 {
            if solution_1.1 < solution.1 {
                solution.1 = solution_1.1;
            }
        }
    }
    if can_go_in_direction(grid, current_pos, Direction::Left) {
        let solution_1 = start_path(
            grid,
            (current_pos.0, current_pos.1 - 1),
            end_pos,
            number_of_steps + 1,
            coordinates_ive_been_to.clone(),
            &mut solutions,
            &mut coordinate_to_steps_map,
        );
        if solution_1.0 {
            if solution_1.1 < solution.1 {
                solution.1 = solution_1.1;
            }
        }
    }

    return solution;
}

fn can_go_in_direction(
    grid: &Vec<Vec<char>>,
    current_pos: (usize, usize),
    direction: Direction,
) -> bool {
    let mut can_go = false;
    match direction {
        Direction::Right => {
            if current_pos.1 + 1 < grid[current_pos.0].len() {
                if grid[current_pos.0][current_pos.1 + 1] as i32
                    - grid[current_pos.0][current_pos.1] as i32
                    <= 1
                {
                    can_go = true;
                }
            }
        }
        Direction::Left => {
            if current_pos.1 > 0 {
                if grid[current_pos.0][current_pos.1 - 1] as i32
                    - grid[current_pos.0][current_pos.1] as i32
                    <= 1
                {
                    can_go = true;
                }
            }
        }

        Direction::Up => {
            if current_pos.0 > 0 {
                if grid[current_pos.0 - 1][current_pos.1] as i32
                    - grid[current_pos.0][current_pos.1] as i32
                    <= 1
                {
                    can_go = true;
                }
            }
        }

        Direction::Down => {
            if current_pos.0 + 1 < grid.len() {
                if grid[current_pos.0 + 1][current_pos.1] as i32
                    - grid[current_pos.0][current_pos.1] as i32
                    <= 1
                {
                    can_go = true;
                }
            }
        }
    }

    return can_go;
}
