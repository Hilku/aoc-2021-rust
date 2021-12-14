use std::collections::HashMap;
fn main() {
    //let input = include_str!("day14smollinput.txt");
    let input = include_str!("day14input.txt");
    let mut parts = input.lines();

    let mut polymer_map: HashMap<String, String> = HashMap::new();
    while let Some(line) = parts.next() {
        let mut polymer_parts = line.split(" -> ");
        let part1 = polymer_parts.next().unwrap();
        let part2 = polymer_parts.next().unwrap();
        polymer_map.insert(part1.to_string(), part2.to_string());
    }

    //let input_str = "NNCB".to_string();
    let input_str = "KHSNHFKVVSVPSCVHBHNP".to_string();

    let input_str_clone = input_str.clone();

    let mut chars_to_check: HashMap<String, usize> = HashMap::new();
    for c in 0..input_str_clone.len() - 1 {
        let mut string_to_check: String = "".to_string();
        string_to_check.insert(0, input_str_clone.chars().nth(c).unwrap());
        string_to_check.insert(1, input_str_clone.chars().nth(c + 1).unwrap());
        chars_to_check.insert(string_to_check, 1);
    }
    let mut chars_to_check_buffer = chars_to_check.clone();
    for step in 0..40 {
        let chars_to_check_clone = chars_to_check_buffer.clone();
        chars_to_check_buffer = HashMap::new();
        for (stuff_to_check, count) in chars_to_check_clone.iter() {
            let mut new_string_to_check: String = "".to_string();
            let new_char0 = stuff_to_check.chars().nth(0).unwrap();
            let new_char1 = polymer_map[stuff_to_check].chars().nth(0).unwrap();
            new_string_to_check.insert(0, new_char0);
            new_string_to_check.insert(1, new_char1);
            match chars_to_check_buffer.get_mut(&new_string_to_check) {
                Some(v) => {
                    *v += count;
                }
                None => {
                    chars_to_check_buffer.insert(new_string_to_check, *count);
                }
            }

            let mut new_string_to_check: String = "".to_string();
            let new_char0 = polymer_map[stuff_to_check].chars().nth(0).unwrap();
            let new_char1 = stuff_to_check.chars().nth(1).unwrap();
            new_string_to_check.insert(0, new_char0);
            new_string_to_check.insert(1, new_char1);
            match chars_to_check_buffer.get_mut(&new_string_to_check) {
                Some(v) => {
                    *v += count;
                }
                None => {
                    chars_to_check_buffer.insert(new_string_to_check, *count);
                }
            }
        }
        if step == 9 {
            println!("PART 1: ");
            count_chars_lowest_highest(&chars_to_check_buffer);
        }
    }
    println!("Part 2: ");
    count_chars_lowest_highest(&chars_to_check_buffer);
}

fn count_chars_lowest_highest(map: &HashMap<String, usize>) {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for (string_to_count, count) in map.iter() {
        for c in string_to_count.chars() {
            let is_in_map = char_counts.get_mut(&c);
            match is_in_map {
                Some(v) => {
                    *v += *count;
                }
                None => {
                    char_counts.insert(c, *count);
                }
            }
        }
    }

    let mut highest: usize = 0;
    let mut lowest: usize = 10000000000000000;
    for (_, count) in char_counts.iter() {
        if *count > highest {
            highest = *count;
        }
        if *count < lowest {
            lowest = *count;
        }
    }

    println!(
        "Highest: {}, Lowest: {}, Highest - Lowest: {}",
        highest,
        lowest,
        ((highest - lowest) as f64 / 2.).ceil()
    );
}
