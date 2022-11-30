use std::collections::HashMap;
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}
#[derive(Debug)]
enum FoldInstruction {
    FoldX(i32),
    FoldY(i32),
}

fn main() {
    let input = include_str!("day13input.txt");
    let fold_input = include_str!("day13foldinput.txt");
    let mut parts = input.lines();

    let mut paper: HashMap<Vec2, bool> = HashMap::new();
    while let Some(line) = parts.next() {
        let mut split_line = line.split(",");
        let mut index = (0, 0);
        let mut counter = 0;
        while let Some(part) = split_line.next() {
            let value = part.parse::<i32>().unwrap();
            if counter == 0 {
                index.0 = value;
                counter += 1;
            } else {
                paper.insert(
                    Vec2 {
                        x: index.0,
                        y: value,
                    },
                    true,
                );
                counter = 0;
            }
        }
    }

    let mut instructions: Vec<FoldInstruction> = Vec::new();
    let mut parts = fold_input.lines();
    while let Some(line) = parts.next() {
        let mut split_line = line.split("fold along ");
        while let Some(part) = split_line.next() {
            let mut new_parts = part.split("=");
            let mut is_x = false;
            let mut value = 0;
            let mut first_pass = 0;
            while let Some(part) = new_parts.next() {
                if first_pass == 0 {
                    is_x = part == "x";
                    first_pass += 1;
                } else {
                    value = part.parse::<i32>().unwrap();
                    first_pass += 1;
                }
            }
            if first_pass == 2 {
                if is_x {
                    instructions.push(FoldInstruction::FoldX(value));
                } else {
                    instructions.push(FoldInstruction::FoldY(value));
                }
            }
        }
    }

    let mut paper_copy = paper.clone();
    let mut fold_counter = 0;
    for instruction in instructions {
        fold_counter += 1;
        let mut old_coordinates: Vec<Vec2> = Vec::new();
        let mut new_coordinates: Vec<Vec2> = Vec::new();
        match instruction {
            FoldInstruction::FoldX(value) => {
                for (coordinate, is_dot) in paper_copy.iter() {
                    if *is_dot {
                        if coordinate.x > value {
                            old_coordinates.push(coordinate.clone());
                            new_coordinates.push(Vec2 {
                                y: coordinate.y,
                                x: value - (coordinate.x - value),
                            })
                        }
                    }
                }
            }
            FoldInstruction::FoldY(value) => {
                for (coordinate, is_dot) in paper_copy.iter() {
                    if *is_dot {
                        if coordinate.y > value {
                            old_coordinates.push(coordinate.clone());
                            new_coordinates.push(Vec2 {
                                x: coordinate.x,
                                y: value - (coordinate.y - value),
                            })
                        }
                    }
                }
            }
        }

        for old_c in old_coordinates.iter() {
            paper_copy.remove(&old_c);
        }
        for new_c in new_coordinates.iter() {
            paper_copy.insert(new_c.clone(), true);
        }
        println!("Fold: {}, Dots: {}", fold_counter, paper_copy.len());
    }
    let mut display: Vec<Vec<char>> = vec![vec![' '; 40]; 40];

    for (coordinate, _) in paper_copy.iter() {
        display[coordinate.x as usize][coordinate.y as usize] = 'X';
    }
    //displayed sideways :|
    for line in display.iter() {
        let mut disp_line: String = "".to_string();
        for c in line.iter() {
            disp_line.push(*c);
            disp_line.push(' ');
        }
        println!("{}", disp_line);
    }
    //displayed correctly
    for i in 0..8 {
        let mut disp_line: String = "".to_string();
        for j in 0..40 {
            disp_line.push(display[j][i]);
            disp_line.push(' ');
        }
        if disp_line.len() > 0 {
            println!("{}", disp_line);
        }
    }
}
