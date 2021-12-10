fn main() {
    let input = include_str!("day10input.txt");
    let mut parts = input.lines();
    let mut chunk_opening: Vec<char> = Vec::new();
    let mut filtered_lines: Vec<String> = Vec::new();
    let mut syntax_error_sum: i32 = 0;

    //part 1
    while let Some(line) = parts.next() {
        let out_slice: &str = &line[..];
        let chars: Vec<char> = out_slice.chars().collect();

        let mut valid_line: bool = true;
        for c in chars.iter() {
            match c {
                //chunk opening
                '(' => {
                    chunk_opening.push('(');
                }
                '[' => {
                    chunk_opening.push('[');
                }
                '{' => {
                    chunk_opening.push('{');
                }
                '<' => {
                    chunk_opening.push('<');
                }
                //chunk closing
                ')' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '(' {
                                syntax_error_sum += 3;
                                valid_line = false;
                                break;
                            }
                        }
                    }
                }
                ']' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '[' {
                                syntax_error_sum += 57;
                                valid_line = false;
                                break;
                            }
                        }
                    }
                }
                '}' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '{' {
                                syntax_error_sum += 1197;
                                valid_line = false;
                                break;
                            }
                        }
                    }
                }
                '>' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '<' {
                                syntax_error_sum += 25137;
                                valid_line = false;
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if valid_line {
            filtered_lines.push(line.to_string());
        }
    }

    println!("Part 1: {}", syntax_error_sum);

    //part 2
    let mut autocomplete_sum: Vec<usize> = Vec::new();
    for line in filtered_lines.iter() {
        let out_slice: &str = &line[..];
        let chars: Vec<char> = out_slice.chars().collect();

        let mut chunk_opening: Vec<char> = Vec::new();
        for c in chars.iter() {
            match c {
                //chunk opening
                '(' => {
                    chunk_opening.push('(');
                }
                '[' => {
                    chunk_opening.push('[');
                }
                '{' => {
                    chunk_opening.push('{');
                }
                '<' => {
                    chunk_opening.push('<');
                }
                //chunk closing
                ')' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '(' {
                                println!("Syntax error, expected '('");
                                break;
                            }
                        }
                    }
                }
                ']' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '[' {
                                println!("Syntax error, expected ']'");
                                break;
                            }
                        }
                    }
                }
                '}' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '{' {
                                println!("Syntax error, expected curly braces");
                                break;
                            }
                        }
                    }
                }
                '>' => {
                    let last_opening = chunk_opening.pop();
                    match last_opening {
                        None => {
                            println!("no opening :(");
                        }
                        Some(e) => {
                            if e != '<' {
                                println!("Syntax error, expected '('");
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        let mut sum: usize = 0;
        while let Some(opening) = chunk_opening.pop() {
            sum *= 5;
            match opening {
                '(' => {
                    sum += 1;
                }
                '[' => {
                    sum += 2;
                }
                '{' => {
                    sum += 3;
                }
                '<' => {
                    sum += 4;
                }
                _ => {}
            }
        }

        autocomplete_sum.push(sum);
    }

    autocomplete_sum.sort();

    println!(
        "Part2: {}",
        autocomplete_sum[(autocomplete_sum.len() as f32 / 2. - 0.5) as usize]
    );
}
