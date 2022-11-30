use regex::Regex;

fn main() {
    let input = include_str!("day18smollinput.txt");
    let mut parts = input.lines();

    let mut final_number: String = "".to_string();

    while let Some(line) = parts.next() {
        if final_number.len() == 0 {
            final_number = line.to_string();
        } else {
            let mut number_to_reduce = add(final_number.clone(), line.to_string());
            println!("number to reduce is: {}", number_to_reduce);
            let mut reducing = true;
            while reducing {
                let red_res = reduce(number_to_reduce.clone());
                number_to_reduce = red_res.0.clone();
                reducing = red_res.1;
            }
            final_number = number_to_reduce.clone();
        }
    }
    println!("Final is: {}", final_number);

    println!(
        "Magnitude of final is: {}",
        calculate_magnitude(final_number)
    );

    //Part 2
    let mut parts = input.lines();

    let mut highest_magnitude = 0;
    let mut all_lines: Vec<String> = Vec::new();
    while let Some(line) = parts.next() {
        all_lines.push(line.to_string());
    }
    let mut step = 0;
    for line_a in all_lines.iter() {
        for line_b in all_lines.iter() {
            step += 1;
            if line_a == line_b {
                continue;
            }
            let mut number_to_reduce = add(line_a.clone(), line_b.to_string());
            let mut reducing = true;
            while reducing {
                let red_res = reduce(number_to_reduce.clone());
                number_to_reduce = red_res.0.clone();
                reducing = red_res.1;
            }
            let mag = calculate_magnitude(number_to_reduce.clone());
            if mag > highest_magnitude {
                highest_magnitude = mag;
            }
            println!("step is: {}", step);
        }
    }

    println!("Highest mag is: {}", highest_magnitude);
}

fn add(nr1: String, nr2: String) -> String {
    let result = format!("[{},{}]", nr1, nr2);
    return result;
}
//String : result of reducing, bool: whether any reducing happened
fn reduce(nr: String) -> (String, bool) {
    let mut nr_of_parentheses = 0;
    let mut string_to_explode = "".to_string();
    //Find explosions
    let mut char_index = 0;
    for c in nr.chars() {
        char_index += 1;
        if string_to_explode.len() > 0 {
            string_to_explode.push(c);
        }
        if c == '[' {
            nr_of_parentheses += 1;
        }
        if c == ']' {
            nr_of_parentheses -= 1;
            if string_to_explode.len() > 0 {
                return (explode(nr, string_to_explode.clone(), char_index), true);
            }
        }

        if nr_of_parentheses == 5 && string_to_explode.len() == 0 {
            string_to_explode.push(c);
        }
    }
    //find splits
    for number in find_regular_numbers(nr.clone()).iter() {
        if *number > 9 {
            let split_stuff = split(nr.clone(), *number);
            return (split_stuff, true);
        }
    }
    return (nr, false);
}

fn split(nr: String, number: i32) -> String {
    let nr_clone = nr.clone();
    let first_nr = (number as f32 / 2.).floor() as i32;
    let second_nr = (number as f32 / 2.).ceil() as i32;
    let replaced_str = nr_clone.replacen(
        &number.to_string(),
        &format!("[{},{}]", first_nr, second_nr),
        1,
    );
    return replaced_str;
}

fn explode(nr: String, string_to_explode: String, char_index: i32) -> String {
    let twoparts = nr.split_at(char_index as usize - string_to_explode.len());

    let mut second_string_part = twoparts.1.to_string();

    second_string_part = second_string_part.replacen(&string_to_explode, "", 1);

    let numbers = find_regular_numbers(string_to_explode.clone());

    let first_nr = numbers[0];
    let second_nr = numbers[1];
    let mut first_part = find_and_replace_regular_numbers(twoparts.0.to_string(), false, first_nr);
    first_part.push('0');
    let second_part = find_and_replace_regular_numbers(second_string_part, true, second_nr);

    let result = first_part + &second_part;

    return result;
}

fn find_regular_numbers(nr: String) -> Vec<i32> {
    let mut result = Vec::new();

    let re = Regex::new(r"[\[\]]").unwrap();
    let after = re.replace_all(&nr, "");
    let mut after = after.split(",");
    while let Some(nr1) = after.next() {
        if nr1.len() > 0 {
            result.push(nr1.parse::<i32>().unwrap());
        }
    }

    return result;
}

fn find_and_replace_regular_numbers(nr: String, return_first: bool, explosion_nr: i32) -> String {
    let regular_numbers = find_regular_numbers(nr.clone());
    if regular_numbers.len() > 0 {
        if return_first {
            let regular_number = regular_numbers.first().unwrap();
            let regular_number_plus_one = regular_number + explosion_nr;
            let replaced = nr.replacen(
                &regular_number.to_string(),
                &regular_number_plus_one.to_string(),
                1,
            );
            return replaced;
        } else {
            let regular_number = regular_numbers.last().unwrap();
            let regular_number_plus_one = regular_number + explosion_nr;

            let isthisit = nr.rfind(&format!("{}", regular_number));

            if isthisit.is_some() {
                let first_ind = isthisit.unwrap();
                let (not_needed, replace_regular_nr) = nr.split_at(first_ind);
                let replace_regular_nr = replace_regular_nr.replacen(
                    &regular_number.to_string(),
                    &regular_number_plus_one.to_string(),
                    1,
                );

                return not_needed.to_string() + &replace_regular_nr;
            }
        }
    }

    return nr.to_string();
}

fn calculate_magnitude_for_pair(nr: String) -> (usize, bool) {
    let result_vec = find_regular_numbers(nr.clone());

    if result_vec.len() > 1 {
        return (
            result_vec[0] as usize * 3 + result_vec[1] as usize * 2,
            true,
        );
    } else {
        return (0, false);
    }
}

fn calculate_magnitude(nr: String) -> usize {
    let mut result = 0;

    let mut buffer_str = nr.clone();
    let mut replaced_stuff = true;
    while replaced_stuff {
        replaced_stuff = false;
        let old_string = buffer_str.clone();
        let re = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
        for cap in re.captures_iter(&old_string) {
            let calculation_result = calculate_magnitude_for_pair(cap[0].to_string());
            if calculation_result.1 {
                result = calculation_result.0;
                buffer_str = buffer_str.replace(&cap[0].to_string(), &result.to_string());
            }
        }

        if old_string != buffer_str {
            replaced_stuff = true;
        }
    }

    return result;
}
