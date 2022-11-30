#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Vec2 {
    x: i32,
    y: i32,
}
#[derive(Clone, Copy, Debug)]
struct Line {
    start: Vec2,
    end: Vec2,
}
fn main() {
    let input = include_str!("day05input.txt");

    let mut split_lines_input = input.lines();
    let mut lines: Vec<Line> = Vec::new();
    //Input parsing
    while let Some(line) = split_lines_input.next() {
        let mut two_inputs = line.split("->");
        let mut new_line: Line = Line {
            start: Vec2 { x: 0, y: 0 },
            end: Vec2 { x: 0, y: 0 },
        };
        let mut line_index = 0;
        while let Some(split) = two_inputs.next() {
            let mut parts = split.split(",");
            let mut index = 0;
            let mut new_vec: Vec2 = Vec2 { x: 0, y: 0 };
            while let Some(i) = parts.next() {
                let mut parsed_parts = i.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                while let Some(parsed_i) = parsed_parts.next() {
                    if index == 0 {
                        new_vec.x = parsed_i;
                    } else {
                        new_vec.y = parsed_i;
                    }
                    index += 1;
                }
            }
            if line_index == 0 {
                new_line.start = new_vec;
            } else {
                new_line.end = new_vec;
            }
            line_index += 1;
        }
        lines.push(new_line);
    }
    //PART1

    let mut all_coordinates: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for i in 0..lines.len() {
        fill_line(&lines[i], &mut all_coordinates);
    }

    let mut overlaps = 0;
    for i in 0..all_coordinates.len() {
        for j in 0..all_coordinates[i].len() {
            if all_coordinates[i][j] > 1 {
                overlaps += 1;
            }
        }
    }
    println!("PART 1 All overlaps: {}", overlaps);

    //PART2

    let mut all_coordinates_2: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for i in 0..lines.len() {
        fill_line_diagonal_too(&lines[i], &mut all_coordinates_2);
    }

    let mut overlaps = 0;
    for i in 0..all_coordinates_2.len() {
        for j in 0..all_coordinates_2[i].len() {
            if all_coordinates_2[i][j] > 1 {
                overlaps += 1;
            }
        }
    }
    println!("PART 2 All overlaps: {}", overlaps);
}

fn fill_line_diagonal_too(line: &Line, overlapping_coordinates: &mut Vec<Vec<i32>>) {
    let mut x_buffer = line.start.x;
    let mut y_buffer = line.start.y;
    while !(x_buffer == line.end.x && y_buffer == line.end.y) {
        overlapping_coordinates[x_buffer as usize][y_buffer as usize] += 1;
        if x_buffer != line.end.x {
            if x_buffer > line.end.x {
                x_buffer -= 1;
            } else {
                x_buffer += 1;
            }
        }
        if y_buffer != line.end.y {
            if y_buffer > line.end.y {
                y_buffer -= 1;
            } else {
                y_buffer += 1;
            }
        }
    }
    overlapping_coordinates[x_buffer as usize][y_buffer as usize] += 1;
}

fn covered_numbers(start: i32, end: i32) -> Vec<i32> {
    let mut buffer_x = start;
    let mut covered_numbers: Vec<i32> = Vec::new();
    while buffer_x != end {
        covered_numbers.push(buffer_x);

        if end > buffer_x {
            buffer_x += 1;
        } else {
            buffer_x -= 1;
        }
    }
    covered_numbers.push(buffer_x);
    return covered_numbers;
}

fn fill_line(line1: &Line, overlapping_coordinates: &mut Vec<Vec<i32>>) {
    if line1.start.x == line1.end.x {
        for number in covered_numbers(line1.start.y, line1.end.y) {
            overlapping_coordinates[line1.start.x as usize][number as usize] += 1;
        }
    } else if line1.start.y == line1.end.y {
        for number in covered_numbers(line1.start.x, line1.end.x) {
            overlapping_coordinates[number as usize][line1.start.y as usize] += 1;
        }
    }
}
