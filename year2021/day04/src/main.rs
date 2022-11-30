#[derive(Clone, Copy, Debug)]
struct BingoField {
    value: i32,
    marked: bool,
}
#[derive(Clone, Copy, Debug)]
struct BingoCard {
    completed: bool,
    card: [[BingoField; 5]; 5],
}

fn main() {
    let input = include_str!("day4input.txt");

    let mut split_lines_input = input.lines();

    let mut bingo_card: BingoCard = BingoCard {
        completed: false,
        card: [[BingoField {
            value: -1,
            marked: false,
        }; 5]; 5],
    };

    let mut cards: Vec<BingoCard> = Vec::new();
    let mut inputs: Vec<i32> = Vec::new();
    let mut index = 0;
    let mut bingo_card_row_index = 0;
    //Input parsing
    while let Some(line) = split_lines_input.next() {
        if index == 0 {
            let mut parts = line.split(",").map(|s| s.parse::<i32>().unwrap());
            while let Some(i) = parts.next() {
                inputs.push(i);
            }
        } else {
            if line == "" {
                cards.push(bingo_card.clone());
                bingo_card = BingoCard {
                    completed: false,
                    card: [[BingoField {
                        value: -1,
                        marked: false,
                    }; 5]; 5],
                };
                bingo_card_row_index = 0;
            } else {
                let mut parts = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                let mut bingo_card_column_index = 0;
                while let Some(i) = parts.next() {
                    bingo_card.card[bingo_card_row_index][bingo_card_column_index] = BingoField {
                        value: i,
                        marked: false,
                    };
                    bingo_card_column_index += 1;
                }
                bingo_card_row_index += 1;
            }
        }
        index += 1;
    }
    cards.push(bingo_card.clone());

    //PART 1
    let mut part_1_cards = cards.clone();
    'outer: for input in inputs.iter() {
        for mut card in part_1_cards.iter_mut() {
            if mark_number(&mut card, *input) {
                if is_bingo(&mut card) {
                    let unmarked_sum = collect_unmarked_numbers(&card);
                    println!(
                        "PART 1 Solution: 
                    Sum of unmarked numbers on this card: {}
                    Current number: {}
                    Solution: {}",
                        unmarked_sum,
                        input,
                        unmarked_sum * input
                    );
                    break 'outer;
                }
            }
        }
    }

    //PART 2
    let mut part_2_cards = cards.clone();
    let mut number_of_completed_cards = 0;
    let number_of_cards = cards.len();
    for input in inputs.iter() {
        for mut card in part_2_cards.iter_mut() {
            if !card.completed {
                if mark_number(&mut card, *input) {
                    if is_bingo(&mut card) {
                        number_of_completed_cards += 1;
                        if number_of_completed_cards == number_of_cards - 1 {
                            println!("Found the last card!");
                            let unmarked_sum = collect_unmarked_numbers(&card);
                            println!(
                                "Sum of unmarked numbers on this card: {}
                            Current number: {}
                            Solution: {}",
                                unmarked_sum,
                                input,
                                unmarked_sum * input
                            );
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn collect_unmarked_numbers(card: &BingoCard) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for q in 0..5 {
            if !card.card[i][q].marked {
                sum += card.card[i][q].value;
            }
        }
    }

    return sum;
}

fn mark_number(card: &mut BingoCard, number: i32) -> bool {
    let mut marked = false;
    for i in 0..5 {
        for q in 0..5 {
            if card.card[i][q].value == number {
                card.card[i][q].marked = true;
                marked = true;
            }
        }
    }
    return marked;
}

fn is_bingo(card: &mut BingoCard) -> bool {
    //check columns
    for q in 0..5 {
        let mut number_of_marks = 0;
        for i in 0..5 {
            if card.card[q][i].marked {
                number_of_marks += 1;
            }
        }
        if number_of_marks == 5 {
            card.completed = true;
            return true;
        }
    }

    //check rows
    for q in 0..5 {
        let mut number_of_marks = 0;
        for i in 0..5 {
            if card.card[i][q].marked {
                number_of_marks += 1;
            }
        }
        if number_of_marks == 5 {
            card.completed = true;
            return true;
        }
    }

    return false;
}
