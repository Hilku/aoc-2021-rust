fn main() {    
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<i32> = Vec::new();

    //Parsing the string as i32

    let mut current_elf_calories : i32 = 0;

    while let Some(string) = split_stuff.next() {
        let line = string.parse::<i32>();
        if let Ok(asd) = line {
            current_elf_calories += asd;
        }
        else
        {
            parsed_vec.push(current_elf_calories);
            current_elf_calories = 0;
        }
    }

    let mut highest_number : i32 = 0;

    //part1

    for number in parsed_vec.iter()
    {
        if *number > highest_number
        {
            highest_number = *number;
        }
    }

    println!("Highest number:{}", highest_number);

    //part2

    parsed_vec.sort();

    println!("highest 3: {}", parsed_vec.iter().rev().take(3).sum::<i32>());

}
