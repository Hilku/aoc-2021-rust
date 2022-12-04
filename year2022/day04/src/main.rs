

fn main() {
    let input = include_str!("day4input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<i32> = Vec::new();
    let mut score: u32 = 0;
    while let Some(string) = split_stuff.next() {
        let mut split = string.split(",");

        let mut first_of_pair = (0,0);
        let mut second_of_pair= (0,0);
        let mut index = 0;
        for s in split
        {
            let mut part_split = s.split("-");
            let first_index = part_split.next().unwrap().parse::<i32>().unwrap();
            let second_index = part_split.next().unwrap().parse::<i32>().unwrap();
            if index == 0
            {
                first_of_pair.0 = first_index;
                first_of_pair.1 = second_index;
            }
            else
            {
                second_of_pair.0 = first_index;
                second_of_pair.1 = second_index;
            }

            index += 1;
        }
        if first_of_pair.0 <= second_of_pair.0 && first_of_pair.1 >= second_of_pair.1
        {
            score += 1;
        }
        else if first_of_pair.0 >= second_of_pair.0 && first_of_pair.1 <= second_of_pair.1
        {
            score += 1;
        }
    }

    let mut all_pairs :Vec<(i32, i32)>  = Vec::new();
    
    let mut split_stuff = input.lines();
    let mut score2 = 0;
    while let Some(string) = split_stuff.next() {
        let mut split = string.split(",");

        let mut first_of_pair = (0,0);
        let mut second_of_pair= (0,0);
        let mut index = 0;
        for s in split
        {
            let mut part_split = s.split("-");
            let first_index = part_split.next().unwrap().parse::<i32>().unwrap();
            let second_index = part_split.next().unwrap().parse::<i32>().unwrap();
            if index == 0
            {
                first_of_pair.0 = first_index;
                first_of_pair.1 = second_index;
            }
            else
            {
                second_of_pair.0 = first_index;
                second_of_pair.1 = second_index;
            }

            index += 1;
        }
        if first_of_pair.0 <= second_of_pair.0 && first_of_pair.1 >= second_of_pair.1
        {
            score2 += 1;
        }
        else if first_of_pair.0 >= second_of_pair.0 && first_of_pair.1 <= second_of_pair.1
        {
            score2 += 1;
        }
        else if first_of_pair.0 <= second_of_pair.1 && first_of_pair.1 >= second_of_pair.0
        {
            score2 += 1;
        }
        else if first_of_pair.1 >= second_of_pair.0 && first_of_pair.0 <= second_of_pair.1
        {
            score2 += 1;
        }
    }

    println!("nr of pairs: {}", score);
    println!("nr of pairs pt2: {}", score2);
}
