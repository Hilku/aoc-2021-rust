fn main() {
    let input = include_str!("day3input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<i32> = Vec::new();
    let mut score: u32 = 0;
    while let Some(string) = split_stuff.next() {
        let (first, second) = string.split_at(string.len() / 2);

        let mut same_items_vec: Vec<u32> = Vec::new();

        for c in first.chars() {
            if second.contains(c) {
                println!("second contains: {}", c);
                let number;

                if (c as u32) < ('a' as u32) {
                    number = c as u32 + 26 + 32 - ('a' as u32 - 1);
                } else {
                    number = c as u32 - ('a' as u32 - 1);
                }

                println!("second contains: {}", number);

                if !same_items_vec.contains(&number) {
                    same_items_vec.push(number);
                }
            }
        }
        for nr in same_items_vec {
            score += nr;
        }
        println!("first: {}, second: {}", first, second);
    }

    println!("Score: {}", score);

    let mut split_stuff = input.lines();
    let mut same_string = "".to_string();
    let mut group_counter = 0;
    let mut badge_score = 0;
    while let Some(string) = split_stuff.next() {
        if group_counter == 0 {
            same_string = string.to_string();
        } else {
            same_string.retain(|c| string.contains(c));
        }

        if group_counter == 2 {
            let mut same_items_vec: Vec<u32> = Vec::new();
            for c in same_string.chars() {
                println!("second contains: {}", c);
                let number;

                if (c as u32) < ('a' as u32) {
                    number = c as u32 + 26 + 32 - ('a' as u32 - 1);
                } else {
                    number = c as u32 - ('a' as u32 - 1);
                }

                println!("second contains: {}", number);

                if !same_items_vec.contains(&number) {
                    same_items_vec.push(number);
                }
            }
            for nr in same_items_vec {
                badge_score += nr;
            }
            group_counter = 0;
            println!("badge score: {}", badge_score);
            continue;
        }

        group_counter += 1;
    }
}
