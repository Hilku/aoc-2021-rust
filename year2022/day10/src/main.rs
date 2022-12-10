fn main() {
    part_1();

    part_2();
}

fn part_1() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    while let Some(line) = lines.next() {
        if line == "noop" {
            cycle += 1;
            if cycle == 20
                || cycle == 60
                || cycle == 100
                || cycle == 140
                || cycle == 180
                || cycle == 220
            {
                sum += cycle * x;
            }
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "addx" {
                for i in 0..2 {
                    cycle += 1;
                    if cycle == 20
                        || cycle == 60
                        || cycle == 100
                        || cycle == 140
                        || cycle == 180
                        || cycle == 220
                    {
                        sum += cycle * x;
                    }
                }
                x += parts[1].parse::<i32>().unwrap();
            }
        }
    }

    println!("signal strength: {}", sum);
}

fn part_2() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut cycle = 0; //crt index is the same
    let mut x = 1;
    let mut sum = 0;

    let mut drawing: Vec<Vec<char>> = vec![vec!['.'; 40]; 7];

    while let Some(line) = lines.next() {
        if line == "noop" {
            let mut drawing_index_1 = (cycle / 40) as usize;
            let mut drawing_index_2 = cycle % 40;
            if x == drawing_index_2 as i64
                || x == drawing_index_2 as i64 - 1
                || x == drawing_index_2 as i64 + 1
            {
                drawing[drawing_index_1][drawing_index_2] = '#';
            } else {
                drawing[drawing_index_1][drawing_index_2] = '.';
            }
            cycle += 1;
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "addx" {
                for i in 0..2 {
                    let mut drawing_index_1 = (cycle / 40) as usize;
                    let mut drawing_index_2 = cycle % 40;
                    if x == drawing_index_2 as i64
                        || x == drawing_index_2 as i64 - 1
                        || x == drawing_index_2 as i64 + 1
                    {
                        drawing[drawing_index_1][drawing_index_2] = '#';
                    } else {
                        drawing[drawing_index_1][drawing_index_2] = '.';
                    }
                    cycle += 1;
                }
                x += parts[1].parse::<i64>().unwrap();
            }
        }
    }

    for line in drawing {
        println!("{:?}", line);
    }
}
