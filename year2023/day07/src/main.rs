use std::cmp::Ordering;
use std::collections::HashMap;
#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

fn main() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut hands: Vec<Hand> = Vec::new();
    while let Some(string) = split_stuff.next() {
        let split_games: Vec<&str> = string.split_whitespace().collect();
        let mut new_hand: Hand = Hand {
            cards: Vec::new(),
            bid: split_games[1].parse::<usize>().unwrap(),
        };
        for c in split_games[0].chars() {
            new_hand.cards.push(c);
        }

        hands.push(new_hand);
    }

    let mut hands_map: HashMap<usize, Vec<Hand>> = HashMap::new();

    let mut butchered_hands = hands.clone();

    for i in 0..hands.len() {
        let mut current_value_of_hand = 0;

        let mut highest_number_char: (char, usize) = ('J', 0);
        for c in hands[i].cards.iter() {
            if *c == 'J' {
                continue;
            }

            let number = hands[i].cards.iter().filter(|&n| *n == *c).count();
            if number > highest_number_char.1 {
                highest_number_char = (*c, number);
            }
        }

        for c in butchered_hands[i].cards.iter_mut() {
            if *c == 'J' {
                *c = highest_number_char.0;
            }
        }

        for c in hands[i].cards.iter() {
            let before_length = butchered_hands[i].cards.len();
            butchered_hands[i].cards.retain(|x| *x != *c);
            let after_length = butchered_hands[i].cards.len();

            if before_length - after_length == 5 {
                current_value_of_hand += 15;
            }

            if before_length - after_length == 4 {
                current_value_of_hand += 10;
            }

            if before_length - after_length == 3 {
                current_value_of_hand += 5;
            }

            if before_length - after_length == 2 {
                current_value_of_hand += 2;
            }
        }

        match hands_map.get_mut(&current_value_of_hand) {
            Some(v) => {
                v.push(hands[i].clone());
            }
            None => {
                hands_map.insert(current_value_of_hand, vec![hands[i].clone()]);
            }
        }
    }

    let mut values_to_check: Vec<usize> = Vec::new();
    for (k, values) in hands_map.iter() {
        values_to_check.push(*k);
        println!("card with value: {}", k);
        for v in values {
            println!("{:?}", v);
        }
        println!("-------------------------");
    }

    values_to_check.sort();
    println!("values_to_check: {:?}", values_to_check);

    let mut final_sorted_vector: Vec<Hand> = Vec::new();
    for value in values_to_check {
        let mut vector_with_numbers: Vec<Hand> = Vec::new();
        match hands_map.get_mut(&value) {
            Some(v) => {
                for card in v.iter() {
                    vector_with_numbers.push(card.clone());
                }
            }
            None => {
                println!("checking value but no card!")
            }
        }

        vector_with_numbers.sort_by(|a, b| {
            for q in 0..a.cards.len() {
                let a_num = convert_char_to_number(a.cards[q]);
                let b_num = convert_char_to_number(b.cards[q]);
                let res = a_num.cmp(&b_num);
                if res != Ordering::Equal {
                    return res;
                }
            }

            println!("THEY ARE EQUAL!!!!!");
            return Ordering::Equal;
        });
        println!("sorted vector:");
        for v in &vector_with_numbers {
            println!("{:?}", v);
        }
        for hand in vector_with_numbers.iter() {
            final_sorted_vector.push(hand.clone());
        }
    }

    let mut sum = 0;
    for i in 0..final_sorted_vector.len() {
        sum += (i + 1) * final_sorted_vector[i].bid;
    }
    println!("sum: {:?}", sum);
}

fn convert_char_to_number(c: char) -> usize {
    if c == 'A' {
        return 14;
    }
    if c == 'K' {
        return 13;
    }
    if c == 'Q' {
        return 12;
    }
    if c == 'J' {
        return 1;
    }
    if c == 'T' {
        return 10;
    }

    if c.is_digit(10) {
        return c.to_digit(10).unwrap() as usize;
    }

    return 0;
}
