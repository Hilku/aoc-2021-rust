use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut sequences: Vec<VecDeque<i64>> = Vec::new();

    while let Some(string) = split_stuff.next() {
        let mut sequence: VecDeque<i64> = VecDeque::new();

        let split_line: Vec<&str> = string.split_whitespace().collect();

        for strin in split_line {
            sequence.push_back(strin.parse::<i64>().unwrap());
        }
        sequences.push(sequence);
    }

    let mut sum = 0;
    for s in sequences {
        let mut all_lines_in_sequence: Vec<VecDeque<i64>> = Vec::new();
        all_lines_in_sequence.push(s.clone());

        let mut reached_all_zero = false;
        while !reached_all_zero {
            let mut diff_vec: VecDeque<i64> = VecDeque::new();
            reached_all_zero = true;
            for i in 0..all_lines_in_sequence[all_lines_in_sequence.len() - 1].len() - 1 {
                let diff = all_lines_in_sequence.last().unwrap()[i + 1]
                    - all_lines_in_sequence.last().unwrap()[i];

                if all_lines_in_sequence.last().unwrap()[i + 1] != 0 {
                    reached_all_zero = false;
                }

                diff_vec.push_back(diff);
            }

            if !reached_all_zero {
                all_lines_in_sequence.push(diff_vec);
            }
        }

        all_lines_in_sequence.reverse();
        for i in 0..all_lines_in_sequence.len() - 1 {
            let index = 0;
            let number_to_push =
                &all_lines_in_sequence[i + 1][index] - &all_lines_in_sequence[i][index];
            all_lines_in_sequence[i + 1].push_front(number_to_push);
        }

        sum += all_lines_in_sequence.last().unwrap().front().unwrap();
    }

    println!("part2 sum: {}", sum);
}

fn part1() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut sequences: Vec<Vec<i64>> = Vec::new();

    while let Some(string) = split_stuff.next() {
        let mut sequence: Vec<i64> = Vec::new();

        let split_line: Vec<&str> = string.split_whitespace().collect();

        for strin in split_line {
            sequence.push(strin.parse::<i64>().unwrap());
        }
        sequences.push(sequence);
    }

    let mut sum = 0;
    for s in sequences {
        let mut all_lines_in_sequence: Vec<Vec<i64>> = Vec::new();
        all_lines_in_sequence.push(s.clone());

        let mut reached_all_zero = false;
        while !reached_all_zero {
            let mut diff_vec: Vec<i64> = Vec::new();
            reached_all_zero = true;
            for i in 0..all_lines_in_sequence[all_lines_in_sequence.len() - 1].len() - 1 {
                let diff = all_lines_in_sequence[all_lines_in_sequence.len() - 1][i + 1]
                    - all_lines_in_sequence[all_lines_in_sequence.len() - 1][i];
                if diff != 0 {
                    reached_all_zero = false;
                }
                diff_vec.push(diff);
            }

            all_lines_in_sequence.push(diff_vec);
        }

        all_lines_in_sequence.reverse();
        for i in 0..all_lines_in_sequence.len() - 1 {
            let index = all_lines_in_sequence[i].len();
            let number_to_push =
                &all_lines_in_sequence[i][index - 1] + &all_lines_in_sequence[i + 1][index - 1];
            all_lines_in_sequence[i + 1].push(number_to_push);
        }

        sum += all_lines_in_sequence.last().unwrap().last().unwrap();
    }

    println!("part1 sum: {}", sum);
}
