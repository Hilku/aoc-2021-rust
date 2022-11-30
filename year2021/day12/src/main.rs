use std::collections::HashMap;

fn main() {
    let input = include_str!("day12input.txt");
    let mut parts = input.lines();
    let mut paths_map: HashMap<String, Vec<String>> = HashMap::new();

    while let Some(line) = parts.next() {
        let mut split_line = line.split("-");
        let mut line_parts: Vec<String> = Vec::new();
        while let Some(part) = split_line.next() {
            line_parts.push(part.to_string());
        }

        match paths_map.get_mut(&line_parts[0]) {
            Some(v) => {
                v.push(line_parts[1].clone());
            }
            None => {
                paths_map.insert(line_parts[0].clone(), vec![line_parts[1].clone()]);
            }
        }

        match paths_map.get_mut(&line_parts[1]) {
            Some(v) => {
                v.push(line_parts[0].clone());
            }
            None => {
                paths_map.insert(line_parts[1].clone(), vec![line_parts[0].clone()]);
            }
        }
    }

    let mut paths: Vec<Vec<String>> = vec![Vec::new()];
    paths[0].push("start".to_string());

    let mut found_new_path = true;
    let mut sum = 0;
    while found_new_path {
        let something = find_paths_part1(&paths, &paths_map);
        found_new_path = something.1;
        paths = something.0.clone();
        sum += something.2;
    }

    println!(
        "Part 1 
    Paths: {:?}",
        sum
    );

    let mut paths: Vec<(Vec<String>, bool)> = vec![(Vec::new(), false)];
    paths[0].0.push("start".to_string());

    let mut found_new_path = true;
    let mut sum = 0;
    while found_new_path {
        let something = find_paths_part2(&paths, &paths_map);
        found_new_path = something.1;
        paths = something.0.clone();
        sum += something.2;
    }

    println!(
        "Part 2 
    Paths: {:?}",
        sum
    );
}

fn find_paths_part1(
    paths: &Vec<Vec<String>>,
    paths_map: &HashMap<String, Vec<String>>,
) -> (Vec<Vec<String>>, bool, usize) {
    let mut new_paths: Vec<Vec<String>> = vec![Vec::new()];
    let mut found_new_path: bool = false;
    let mut new_ended_paths: usize = 0;
    for path in paths.iter() {
        if path.len() > 0 {
            match paths_map.get(path.get(path.len() - 1).unwrap()) {
                Some(possible_next_steps) => {
                    for possible_step in possible_next_steps.iter() {
                        if !path.contains(&"end".to_string()) {
                            if possible_step.chars().all(char::is_lowercase) {
                                if !path.contains(&possible_step) {
                                    if possible_step == "end" {
                                        new_ended_paths += 1;
                                    }
                                    let mut path_clone = path.clone();
                                    path_clone.push(possible_step.clone());
                                    new_paths.push(path_clone.clone());
                                    found_new_path = true;
                                }
                            } else {
                                let mut path_clone = path.clone();
                                path_clone.push(possible_step.clone());
                                new_paths.push(path_clone.clone());
                                found_new_path = true;
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }
    if !found_new_path {
        return (paths.clone(), false, new_ended_paths);
    }
    return (new_paths, found_new_path, new_ended_paths);
}

fn find_paths_part2(
    paths: &Vec<(Vec<String>, bool)>,
    paths_map: &HashMap<String, Vec<String>>,
) -> (Vec<(Vec<String>, bool)>, bool, usize) {
    let mut new_paths: Vec<(Vec<String>, bool)> = vec![(Vec::new(), false)];
    let mut found_new_path: bool = false;
    let mut new_ended_paths: usize = 0;
    for path in paths.iter() {
        if path.0.len() > 0 {
            match paths_map.get(path.0.get(path.0.len() - 1).unwrap()) {
                Some(possible_next_steps) => {
                    for possible_step in possible_next_steps.iter() {
                        if !path.0.contains(&"end".to_string()) && possible_step != "start" {
                            if possible_step.chars().all(char::is_lowercase) {
                                if path.1 && !path.0.contains(&possible_step) {
                                    if possible_step == "end" {
                                        new_ended_paths += 1;
                                    }
                                    let mut path_clone = path.clone();
                                    path_clone.0.push(possible_step.clone());
                                    new_paths.push(path_clone.clone());
                                    found_new_path = true;
                                } else if !path.1 {
                                    if possible_step == "end" {
                                        new_ended_paths += 1;
                                    }
                                    let mut path_clone = path.clone();
                                    if path.0.contains(&possible_step) {
                                        path_clone.1 = true;
                                    }
                                    path_clone.0.push(possible_step.clone());
                                    new_paths.push(path_clone.clone());
                                    found_new_path = true;
                                }
                            } else {
                                let mut path_clone = path.clone();
                                path_clone.0.push(possible_step.clone());
                                new_paths.push(path_clone.clone());
                                found_new_path = true;
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }
    if !found_new_path {
        return (paths.clone(), false, new_ended_paths);
    }
    return (new_paths, found_new_path, new_ended_paths);
}
