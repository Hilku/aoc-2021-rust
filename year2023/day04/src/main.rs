use std::collections::HashMap;

#[derive(Clone)]
struct Card {
    winning_numbers: Vec<i64>,
    my_numbers: Vec<i64>,
}

fn main() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<Card> = Vec::new();

    while let Some(string) = split_stuff.next() {
        let split_games: Vec<&str> = string.split(':').collect();
        let split_sets: Vec<&str> = split_games[1].split('|').collect();
        let winning_numbers: Vec<&str> = split_sets[0].split_whitespace().collect();
        let my_nubmers: Vec<&str> = split_sets[1].split_whitespace().collect();

        let mut card = Card {
            winning_numbers: Vec::new(),
            my_numbers: Vec::new(),
        };

        for number in winning_numbers.iter() {
            card.winning_numbers.push(number.parse::<i64>().unwrap());
        }
        for number in my_nubmers.iter() {
            card.my_numbers.push(number.parse::<i64>().unwrap());
        }
        parsed_vec.push(card);
    }
    let mut sum = 0;
    for card in parsed_vec.iter() {
        let mut matches = 0;
        for number in card.winning_numbers.iter() {
            if card.my_numbers.contains(&number) {
                if matches == 0 {
                    matches = 1;
                } else {
                    matches *= 2;
                }
            }
        }

        sum += matches;
    }

    println!("part 1 sum is: {}", sum);
    let mut card_count = parsed_vec.len();
    let mut copies_indexes: Vec<usize> = Vec::new();

    for i in 0..parsed_vec.len() {
        let mut matches = 0;
        for number in parsed_vec[i].winning_numbers.iter() {
            if parsed_vec[i].my_numbers.contains(&number) {
                matches += 1;
            }
        }
        for q in i + 1..i + 1 + matches {
            if q < parsed_vec.len() {
                copies_indexes.push(q);
            }
        }
    }

    while copies_indexes.len() > 0 {
        card_count += 1;
        let i = copies_indexes.pop().unwrap();
        let mut matches = 0;
        for number in parsed_vec[i].winning_numbers.iter() {
            if parsed_vec[i].my_numbers.contains(&number) {
                matches += 1;
            }
        }
        let mut index_count = 0;
        for q in i + 1..i + 1 + matches {
            if q < parsed_vec.len() {
                index_count += 1;
                copies_indexes.push(q);
            }
        }
    }

    println!("parsed vec len: {}", card_count);
}
