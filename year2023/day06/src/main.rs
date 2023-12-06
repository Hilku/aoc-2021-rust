use std::collections::HashMap;

fn main() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut time: Vec<i64> = Vec::new();
    let mut distances_needed: Vec<i64> = Vec::new();

    let mut index = 0;
    while let Some(string) = split_stuff.next() {
        let numbers_merged = string.replace(" ", "");
        let split_games: Vec<&str> = numbers_merged.split(":").collect();
        for number in split_games.iter() {
            if let Ok(num) = number.parse::<i64>() {
                if index == 0 {
                    time.push(num);
                } else {
                    distances_needed.push(num)
                }
            }
        }
        index += 1;
    }

    let mut end_vector: Vec<i64> = Vec::new();
    for i in 0..time.len() {
        end_vector.push(0);
        for holding_for in 0..time[i] {
            if (time[i] - holding_for) * holding_for > distances_needed[i] {
                end_vector[i] += 1;
            }
        }
    }

    let mut sum = 1;
    for num in end_vector.iter() {
        sum *= num;
    }

    println!("sum: {}", sum);
}
