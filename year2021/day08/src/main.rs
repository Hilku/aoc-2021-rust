#[derive(Debug, Clone)]
struct Display {
    segments: Vec<String>,
    output: Vec<String>,
    numbers: Vec<String>,
}

fn main() {
    //INPUT PARSING
    //let input = include_str!("day8smollinput.txt");
    let input = include_str!("day8input.txt");
    let mut lines = input.lines();
    let mut displays: Vec<Display> = Vec::new();
    while let Some(line) = lines.next() {
        let mut columns = line.split(" | ");
        let mut display: Display = Display {
            output: Vec::new(),
            segments: Vec::new(),
            numbers: vec!["".to_string(); 10],
        };

        let mut column_index = 0;

        while let Some(column) = columns.next() {
            let mut parts = column.split_whitespace();
            while let Some(part) = parts.next() {
                let out_slice: &str = &part[..];
                let mut chars: Vec<char> = out_slice.chars().collect();
                chars.sort_by(|a, b| b.cmp(a));
                let sorted_out = String::from_iter(chars);

                if column_index == 0 {
                    display.segments.push(sorted_out);
                } else {
                    display.output.push(sorted_out);
                }
            }
            column_index += 1;
        }
        displays.push(display);
    }
    //part 1
    let mut sum = 0;

    for display in displays.iter() {
        sum += count_unique_digits(&display);
    }

    println!(
        "PART 1
    Unique digits: {}",
        sum
    );

    //Part 2
    let mut end_number = 0;
    for mut display in displays.iter_mut() {
        solve_stuff(&mut display);
        let mut index = 1000;

        let output_copy = display.numbers.clone();
        println!("display.numbers: {:?}", display.numbers);
        for out in display.output.iter() {
            println!("out: {}", out);
            if *out == output_copy[0] {
                end_number += index * 0;
            } else if *out == output_copy[1] {
                end_number += index;
            } else if *out == output_copy[2] {
                end_number += index * 2;
            } else if *out == output_copy[3] {
                end_number += index * 3;
            } else if *out == output_copy[4] {
                end_number += index * 4;
            } else if *out == output_copy[5] {
                end_number += index * 5;
            } else if *out == output_copy[6] {
                end_number += index * 6;
            } else if *out == output_copy[7] {
                end_number += index * 7;
            } else if *out == output_copy[8] {
                end_number += index * 8;
            } else if *out == output_copy[9] {
                end_number += index * 9;
            }
            index /= 10;
        }
        println!("Current sum = {}", end_number);
    }
    println!("Sum of all end numbers: {}", end_number);
}
//0 len 6
//1 == cf len 2
//2 acdeg len 5
//3 acdfg len 5
//4 ==  bcdf len 4
//5 = abdfg len 5
//6 == abdefg len 6
//7 ==  acf len 3
//8 == abcdefg len 7
//9 = abcdfg len 6

fn count_unique_digits(display: &Display) -> usize {
    let mut count: usize = 0;

    for out in display.output.iter() {
        match out.len() {
            //Digit 1
            2 => {
                count += 1;
            }
            //Digit 7
            3 => {
                count += 1;
            }
            //Digit 4
            4 => {
                count += 1;
            }
            //Digit 8
            7 => {
                count += 1;
            }
            _ => {}
        }
    }

    return count;
}

fn deduce_numbers(display: &mut Display, strings: &Vec<String>) -> bool {
    let mut solved_number = false;
    for string in strings.iter() {
        match string.len() {
            //Digit 1
            2 => {
                if display.numbers[1] == "" {
                    solved_number = true;
                    display.numbers[1] = string.to_string();
                }
            }
            //Digit 7
            3 => {
                if display.numbers[7] == "" {
                    solved_number = true;
                    display.numbers[7] = string.to_string();
                }
            }
            //Digit 4
            4 => {
                if display.numbers[4] == "" {
                    solved_number = true;
                    display.numbers[4] = string.to_string();
                }
            }
            //Digit 8
            7 => {
                if display.numbers[8] == "" {
                    solved_number = true;
                    display.numbers[8] = string.to_string();
                }
            }
            //Digits: 2*, 3*, 5*
            5 => {
                //Deducing number 3
                let digit1 = display.numbers[1].as_str();

                if digit1 != "" {
                    if string.contains(digit1.as_bytes()[0] as char)
                        && string.contains(digit1.as_bytes()[1] as char)
                    {
                        //This is number3
                        if display.numbers[3] == "" {
                            solved_number = true;
                            display.numbers[3] = string.to_string();
                        }
                    } else {
                        let digit4 = display.numbers[4].as_str();
                        if digit4 != "" {
                            let mut egyezes = 0;
                            for i in 0..4 {
                                if string.contains(digit4.as_bytes()[i] as char) {
                                    egyezes += 1;
                                }
                            }

                            if egyezes == 2 {
                                if display.numbers[2] == "" {
                                    solved_number = true;
                                    display.numbers[2] = string.to_string();
                                }
                            } else if egyezes == 3 {
                                if display.numbers[5] == "" {
                                    solved_number = true;
                                    display.numbers[5] = string.to_string();
                                }
                            }
                        }
                    }
                }
            }
            //Digits, 0, 6*, 9
            6 => {
                let digit1 = display.numbers[1].as_str();
                if digit1 != "" {
                    let mut egyezes = 0;
                    for i in 0..2 {
                        if string.contains(digit1.as_bytes()[i] as char) {
                            egyezes += 1;
                        }
                    }
                    if egyezes == 1 {
                        if display.numbers[6] == "" {
                            display.numbers[6] = string.to_string();
                            solved_number = true;
                        }
                    } else {
                        let digit4 = display.numbers[4].as_str();
                        if digit4 != "" {
                            let mut egyezes = 0;
                            for i in 0..4 {
                                if string.contains(digit4.as_bytes()[i] as char) {
                                    egyezes += 1;
                                }
                            }

                            if egyezes == 4 {
                                if display.numbers[9] == "" {
                                    solved_number = true;
                                    display.numbers[9] = string.to_string();
                                }
                            } else if egyezes == 3 {
                                if display.numbers[0] == "" {
                                    solved_number = true;
                                    display.numbers[0] = string.to_string();
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return solved_number;
}

fn solve_stuff(mut display: &mut Display) {
    let mut solved_number = true;
    while solved_number {
        solved_number = false;
        let outputs = display.output.clone();
        let segments = display.segments.clone();

        solved_number = solved_number || deduce_numbers(&mut display, &outputs);
        solved_number = solved_number || deduce_numbers(&mut display, &segments);
    }
}
