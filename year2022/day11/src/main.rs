#[derive(Clone, Debug)]
enum NumberVariable {
    Number(usize),
    Variable,
}

#[derive(Clone, Debug)]
enum OperationEnum {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
struct Operation {
    a: NumberVariable,
    b: NumberVariable,
    operation: OperationEnum,
}
#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible: usize,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut line_index = 0;
    while let Some(line) = lines.next() {
        if line_index == 0 {
            monkeys.push(Monkey {
                items: Vec::new(),
                operation: Operation {
                    a: NumberVariable::Number(0),
                    b: NumberVariable::Number(0),
                    operation: OperationEnum::Add,
                },
                test_divisible: 1,
                if_true_throw_to: 1,
                if_false_throw_to: 0,
            });
        }
        let last_monkey_index = monkeys.len() - 1;

        if line_index == 1 {
            let split_first_word: Vec<&str> = line.split("Starting items: ").collect();
            let without_comma: Vec<&str> = split_first_word[1].split(",").collect();
            for without_comma_words in without_comma {
                let without_spaces: Vec<&str> = without_comma_words.split_whitespace().collect();
                for word in without_spaces {
                    monkeys[last_monkey_index]
                        .items
                        .push(word.parse::<usize>().unwrap());
                }
            }
        }
        if line_index == 2 {
            let split_first_word: Vec<&str> = line.split("Operation: new = ").collect();

            let without_spaces: Vec<&str> = split_first_word[1].split_whitespace().collect();

            if without_spaces[0] == "old" {
                monkeys[last_monkey_index].operation.a = NumberVariable::Variable;
            } else {
                monkeys[last_monkey_index].operation.a =
                    NumberVariable::Number(without_spaces[0].parse::<usize>().unwrap());
            }

            if without_spaces[1] == "*" {
                monkeys[last_monkey_index].operation.operation = OperationEnum::Multiply;
            } else if without_spaces[1] == "+" {
                monkeys[last_monkey_index].operation.operation = OperationEnum::Add;
            }

            if without_spaces[2] == "old" {
                monkeys[last_monkey_index].operation.b = NumberVariable::Variable;
            } else {
                monkeys[last_monkey_index].operation.b =
                    NumberVariable::Number(without_spaces[2].parse::<usize>().unwrap());
            }
        }

        if line_index == 3 {
            let split_first_word: Vec<&str> = line.split("Test: divisible by ").collect();
            monkeys[last_monkey_index].test_divisible =
                split_first_word[1].parse::<usize>().unwrap();
        }
        if line_index == 4 {
            let split_first_word: Vec<&str> = line.split("If true: throw to monkey ").collect();
            monkeys[last_monkey_index].if_true_throw_to =
                split_first_word[1].parse::<usize>().unwrap();
        }
        if line_index == 5 {
            let split_first_word: Vec<&str> = line.split("If false: throw to monkey ").collect();
            monkeys[last_monkey_index].if_false_throw_to =
                split_first_word[1].parse::<usize>().unwrap();
        }

        line_index += 1;

        if line_index == 7 {
            line_index = 0;
        }
    }

    let mut number_of_times_inspected: Vec<usize> = vec![0; monkeys.len()];

    let mut multiplies_that_matter: Vec<usize> = Vec::new();
    let mut multiplication_of_multiplies_that_matter = 1;
    for monkey in &monkeys {
        if !multiplies_that_matter.contains(&monkey.test_divisible) {
            multiplies_that_matter.push(monkey.test_divisible);
            multiplication_of_multiplies_that_matter *= monkey.test_divisible;
        }
    }

    for iteration in 0..10000 {
        if iteration % 1000 == 0 || iteration == 20 || iteration == 1 {
            println!("iteration: {}", iteration);
            let mut clone = number_of_times_inspected.clone();
            clone.sort();
            clone.reverse();
            println!(
                "{} * {} :  {:?}, {:?}",
                clone[0],
                clone[1],
                clone[0] * clone[1],
                clone
            );
        }
        for i in 0..monkeys.len() {
            //usize: item, usize: monkey_to_throw_to
            let mut monkey = &mut monkeys[i];
            let mut items_to_throw: Vec<(usize, usize)> = Vec::new();
            for item in monkey.items.clone() {
                number_of_times_inspected[i] += 1;

                let mut worry_level = item;

                let variable_b;
                match monkey.operation.b {
                    NumberVariable::Number(nr) => {
                        variable_b = nr;
                    }
                    NumberVariable::Variable => {
                        variable_b = worry_level;
                    }
                }

                match monkey.operation.operation {
                    OperationEnum::Multiply => {
                        worry_level = worry_level * variable_b;
                    }
                    OperationEnum::Add => {
                        worry_level = worry_level + variable_b;
                    }
                }

                worry_level = worry_level % multiplication_of_multiplies_that_matter;

                /*let division = worry_level / multiplication_of_multiplies_that_matter;

                if division > 2 {
                    worry_level -= (division - 1) * multiplication_of_multiplies_that_matter;
                }*/

                if worry_level % monkey.test_divisible == 0 {
                    items_to_throw.push((worry_level, monkey.if_true_throw_to));
                } else {
                    items_to_throw.push((worry_level, monkey.if_false_throw_to));
                }
            }

            monkey.items = Vec::new();
            for item in items_to_throw {
                monkeys[item.1].items.push(item.0);
            }
        }
    }
    number_of_times_inspected.sort();
    number_of_times_inspected.reverse();
    println!(
        "{} * {} :  {:?}",
        number_of_times_inspected[0],
        number_of_times_inspected[1],
        number_of_times_inspected[0] * number_of_times_inspected[1]
    );
}
