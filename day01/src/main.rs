fn main() {
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();
    let mut previous_number: i32 = 0;
    let mut increased_counter = 0;

    let mut parsed_vec: Vec<i32> = Vec::new();

    //Parsing the string as i32

    while let Some(string) = split_stuff.next() {
        parsed_vec.push(string.parse().unwrap());
    }

    //Task 1
    for number in parsed_vec.iter() {
        if previous_number < *number {
            increased_counter += 1;
        }
        previous_number = *number;
    }

    println!("Solution to first part: {}", increased_counter - 1);
    //Task 2

    let mut sliding_index: usize = 0;
    let mut last_added_values: i32 = 0;
    let mut task2_counter: usize = 0;
    while sliding_index < parsed_vec.len() - 2 {
        let new_added_values = parsed_vec[sliding_index]
            + parsed_vec[sliding_index + 1]
            + parsed_vec[sliding_index + 2];

        if new_added_values > last_added_values {
            task2_counter += 1;
        }
        last_added_values = new_added_values;
        sliding_index += 1;
    }
    println!("Solution to second part: {}", task2_counter - 1);
}
