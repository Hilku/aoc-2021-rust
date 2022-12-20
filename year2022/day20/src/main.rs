use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<i64> = Vec::new();

    let input = include_str!("input.txt");
    let mut lines = input.lines();

    while let Some(big_line) = lines.next() {
        numbers.push(big_line.parse::<i64>().unwrap());
    }

    let mut number_index_map: HashMap<i64, i64> = HashMap::new();

    for i in 0..numbers.len() {
        number_index_map.insert(numbers[i], i.try_into().unwrap());
    }

    //Mix
    for _ in 0..1 {
        for number in &numbers {
            let old_index = number_index_map[number];
            let mut new_index = old_index + number;
            if new_index >= numbers.len() as i64 {
                while new_index >= numbers.len() as i64 {
                    new_index -= numbers.len() as i64;
                }
                new_index += 1;
            }
            if new_index < 0 {
                while new_index < 0 {
                    new_index += numbers.len() as i64;
                }
                new_index -= 1;
            }
            /*println!(
                "number: {}, old_index: {}, new_index: {}",
                number, old_index, new_index
            );*/
            *number_index_map.get_mut(&number).unwrap() = new_index;

            for (key, mut value) in &mut number_index_map {
                if key != number {
                    if new_index > old_index {
                        if *value >= old_index && *value <= new_index {
                            *value -= 1;
                        }
                    }
                    if new_index < old_index {
                        if *value <= old_index && *value >= new_index {
                            *value += 1;
                        }
                    }
                }
            }

            //println!("currnet numberindexmap: {:?}", number_index_map);
        }
    }

    for (k, mut v) in &mut number_index_map {
        *v -= 1;
        if *v < 0 {
            *v = numbers.len() as i64 - 1;
        }
    }

    let zero_index = number_index_map[&0];

    let mut thousand = 0;
    let mut two_thousand = 1;
    let mut three_thousand = 2;

    for (k, v) in number_index_map.clone() {
        if v as usize == (zero_index + 1000) as usize % numbers.len() {
            thousand = k;
        }
        if v as usize == (zero_index + 2000) as usize % numbers.len() {
            two_thousand = k;
        }
        if v as usize == (zero_index + 3000) as usize % numbers.len() {
            three_thousand = k;
        }
    }
    println!("{:?}", number_index_map);
    println!(
        "numbers: {}, {}, {}, sum :{}",
        thousand,
        two_thousand,
        three_thousand,
        thousand + two_thousand + three_thousand
    );
}
