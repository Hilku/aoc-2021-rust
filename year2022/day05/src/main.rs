fn main() {
    let input = include_str!("day5cratesinput.txt");
    let mut lines: Vec<_> = input.lines().collect();
    lines.reverse();
    let mut score: i32 = 0;

    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut index: i32 = 0;
    for string in lines.iter() {
        if index == 0 {
            let mut without_whitespace: String = string.split_whitespace().collect();
            for c in without_whitespace.chars() {
                crates.push(Vec::new());
            }
        } else {
            let mut vec_index = 0;
            let mut char_index = 0;
            for c in string.chars() {
                if char_index == 1 {
                    if c != ' ' {
                        crates[vec_index].push(c);
                    }
                    vec_index += 1;
                }
                char_index += 1;
                if char_index == 4 {
                    char_index = 0;
                }
            }
        }
        index += 1;
    }

    println!("Crates: {:?}", crates);

    let mut commands: Vec<(i32, i32, i32)> = Vec::new();
    let commands_input = include_str!("day5commandsinput.txt");
    let mut lines: Vec<_> = commands_input.lines().collect();
    for line in lines.iter() {
        let mut modified_line = line.replace("move", "");
        modified_line = modified_line.replace("from", "");
        modified_line = modified_line.replace("to", "");
        let without_whitespace = modified_line.split_whitespace();
        let mut new_element = (0, 0, 0);
        let mut index = 0;
        for s in without_whitespace {
            println!("s: {}", s);
            if index == 0 {
                new_element.0 = s.parse::<i32>().unwrap();
            } else if index == 1 {
                new_element.1 = s.parse::<i32>().unwrap();
            } else if index == 2 {
                new_element.2 = s.parse::<i32>().unwrap();
                commands.push(new_element);
            }
            index += 1;
        }
    }

    println!("commands: {:?}", commands);
    let mut modified_crates = crates.clone();
    let mut commands_clone = commands.clone();
    for command in commands_clone {
        for i in 0..command.0 {
            let value = modified_crates[command.1 as usize - 1].pop().unwrap();
            modified_crates[command.2 as usize - 1].push(value);
        }
    }

    println!("Crates: {:?}", modified_crates);
    let mut solution: String = "".to_string();
    for c in &mut modified_crates {
        solution.push(c.pop().unwrap());
    }
    println!("solution part1: {}", solution);

    let mut modified_crates = crates.clone();

    let mut commands_clone = commands.clone();
    for command in commands_clone {
        let mut modified_vector: Vec<char> = Vec::new();
        for i in 0..command.0 {
            let value = modified_crates[command.1 as usize - 1].pop().unwrap();
            modified_vector.push(value);
        }
        modified_vector.reverse();

        for c in modified_vector {
            modified_crates[command.2 as usize - 1].push(c);
        }
    }

    let mut solution: String = "".to_string();
    for c in &mut modified_crates {
        solution.push(c.pop().unwrap());
    }
    println!("solution part2: {}", solution);
}
