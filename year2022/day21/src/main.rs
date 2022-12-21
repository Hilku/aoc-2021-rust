use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Minus,
    Mul,
    Div,
    Equal,
}
#[derive(Clone, Debug)]
struct MonkeyOperation {
    name: String,
    monkey1: String,
    op: Operation,
    monkey2: String,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut simple_monkeys_og: HashMap<String, i64> = HashMap::new();
    let mut complex_monekys_og: Vec<MonkeyOperation> = Vec::new();
    while let Some(big_line) = lines.next() {
        let without_dots = big_line.replace(":", "");
        let without_white_space: Vec<&str> = without_dots.split_whitespace().collect();
        if without_white_space.len() <= 2 {
            simple_monkeys_og.insert(
                without_white_space[0].to_string(),
                without_white_space[1].parse::<i64>().unwrap(),
            );
        } else {
            let mut operation = Operation::Plus;
            if without_white_space[2] == "+" {
                operation = Operation::Plus;
            } else if without_white_space[2] == "-" {
                operation = Operation::Minus;
            } else if without_white_space[2] == "*" {
                operation = Operation::Mul;
            } else if without_white_space[2] == "/" {
                operation = Operation::Div;
            } else if without_white_space[2] == "==" {
                operation = Operation::Equal;
            }
            let new_monkey = MonkeyOperation {
                name: without_white_space[0].to_string(),
                monkey1: without_white_space[1].to_string(),
                op: operation,
                monkey2: without_white_space[3].to_string(),
            };

            complex_monekys_og.push(new_monkey);
        }
    }

    for _ in 0..2000 {
        for i in 0..complex_monekys_og.len() {
            let complex_monkey = complex_monekys_og[i].clone();

            if let Some(monkey1) = simple_monkeys_og.get(&complex_monkey.monkey1) {
                if let Some(monkey2) = simple_monkeys_og.get(&complex_monkey.monkey2) {
                    if complex_monkey.monkey1 == "humn" || complex_monkey.monkey2 == "humn" {
                        continue;
                    }
                    let mut value = 0;
                    match complex_monkey.op {
                        Operation::Plus => {
                            value = monkey1 + monkey2;
                        }
                        Operation::Minus => {
                            value = monkey1 - monkey2;
                        }
                        Operation::Mul => {
                            value = monkey1 * monkey2;
                        }
                        Operation::Div => {
                            value = monkey1 / monkey2;
                        }
                        Operation::Equal => {
                            if monkey1 == monkey2 {}
                            value = 1;
                        }
                    }

                    simple_monkeys_og.insert(complex_monkey.name, value);
                    complex_monekys_og.remove(i);
                    break;
                }
            }
        }
    }
    println!("Complex numberts are: {:?}", complex_monekys_og.len());
    for human_value in 3219579395609 - 10 as i64..10565407195785 + 100 {
        let mut simple_monkeys = simple_monkeys_og.clone();
        let mut complex_monekys = complex_monekys_og.clone();
        *simple_monkeys.get_mut("humn").unwrap() = human_value;
        while complex_monekys.len() > 0 {
            for i in 0..complex_monekys.len() {
                let complex_monkey = complex_monekys[i].clone();

                if let Some(monkey1) = simple_monkeys.get(&complex_monkey.monkey1) {
                    if let Some(monkey2) = simple_monkeys.get(&complex_monkey.monkey2) {
                        let mut value = 0;
                        match complex_monkey.op {
                            Operation::Plus => {
                                value = monkey1 + monkey2;
                            }
                            Operation::Minus => {
                                value = monkey1 - monkey2;
                            }
                            Operation::Mul => {
                                value = monkey1 * monkey2;
                            }
                            Operation::Div => {
                                value = monkey1 / monkey2;
                            }
                            Operation::Equal => {
                                println!(
                                    "checking if eq: {}, {}, {}",
                                    monkey1, monkey2, human_value
                                );
                                if monkey1 == monkey2 {
                                    println!("Yess its eq ! human_value: {}", human_value);
                                    return;
                                }
                                value = 1;
                            }
                        }

                        simple_monkeys.insert(complex_monkey.name, value);
                        complex_monekys.remove(i);
                        break;
                    }
                }
            }
        }
    }
}
