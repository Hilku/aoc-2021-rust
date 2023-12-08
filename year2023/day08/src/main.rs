use std::collections::HashMap;
#[derive(Debug, Clone)]
struct LeftRight {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut input_map: HashMap<String, LeftRight> = HashMap::new();

    let instructions = "LRLRLLRLRLRRLRLRLRRLRLRLLRRLRRLRLRLRLLRRRLRRRLLRRLRLRLRRRLRRLRRRLRLRLRRLRLLRLRLRRLRRRLRLRRLRRRLLRLRLRRRLRRRLRLRRRLRLRRRLLRRLLLRRRLLRRRLRRRLRRRLRLRLRLLRLRRLRLRLLLRRLRRLRRLRLRRLRRLLRRLRLRRRLRLRLLRRRLRRRLRRRLLLRRRLRLRLRRLRRRLRRRLRLRRRLRRLRRRLRLRRLLRRRLRRRLLLRRLRLRLRRLRRRLRRLRRLRLRRRR";

    let mut start_keys: Vec<String> = Vec::new();
    let mut end_keys: Vec<String> = Vec::new();

    while let Some(string) = split_stuff.next() {
        let mut left_right: LeftRight = LeftRight {
            left: "".to_string(),
            right: "".to_string(),
        };

        let first_split: Vec<&str> = string.split(" = ").collect();
        let key: String = first_split[0].to_string();

        if key.chars().collect::<Vec<char>>()[2] == 'A' {
            start_keys.push(key.clone());
        }
        if key.chars().collect::<Vec<char>>()[2] == 'Z' {
            end_keys.push(key.clone());
        }

        let mut replaced = first_split[1].replace("(", "");
        replaced = replaced.replace(")", "");
        replaced = replaced.replace(" ", "");

        let second_split: Vec<&str> = replaced.split(",").collect();
        left_right.left = second_split[0].to_string();
        left_right.right = second_split[1].to_string();

        input_map.insert(key, left_right);
    }

    println!("{:?}", start_keys);
    println!("{:?}", end_keys);
    let mut steps = 0;

    let mut current_key = "AAA".to_string();

    let mut instruction_index = 0;

    let char_vec: Vec<char> = instructions.chars().collect::<Vec<char>>();

    while current_key != "ZZZ" {
        if char_vec[instruction_index] == 'R' {
            current_key = input_map.get(&current_key).unwrap().right.clone();
        } else {
            current_key = input_map.get(&current_key).unwrap().left.clone();
        }
        steps += 1;
        instruction_index += 1;
        if instruction_index >= char_vec.len() {
            instruction_index = 0;
        }
    }

    let mut current_keys: Vec<String> = start_keys.clone();

    let mut standing_on_end_node_count = 0;

    instruction_index = 0;
    let mut steps_part2 = 0;

    let mut steps_to_reach_z: Vec<Vec<usize>> = Vec::new();
    for _ in &current_keys {
        steps_to_reach_z.push(Vec::new());
    }
    while standing_on_end_node_count != current_keys.len() {
        steps_part2 += 1;
        standing_on_end_node_count = 0;
        let mut rando_index = 0;
        for k in current_keys.iter_mut() {
            let k_value = k.clone();
            if char_vec[instruction_index] == 'R' {
                *k = input_map.get(&k_value).unwrap().right.clone();
            } else {
                *k = input_map.get(&k_value).unwrap().left.clone();
            }

            if end_keys.contains(&k) {
                standing_on_end_node_count += 1;
                if !&steps_to_reach_z[rando_index].contains(&steps_part2) {
                    steps_to_reach_z[rando_index].push(steps_part2);
                }
            }

            rando_index += 1;
        }

        let mut should_break = true;

        for value in &steps_to_reach_z {
            if value.len() < 2 {
                should_break = false;
                break;
            }
        }

        if should_break {
            break;
        }

        instruction_index += 1;
        if instruction_index >= char_vec.len() {
            instruction_index = 0;
        }
    }

    println!("steps: {}", steps);
    println!("steps part2: {:?}", steps_to_reach_z);
}

pub fn lcm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
