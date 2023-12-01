fn main() {    
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<i32> = Vec::new();

    //Parsing the string as i32

    let mut current_elf_calories : i32 = 0;

    while let Some(string) = split_stuff.next() {
        let mut replaced_string = string.replace("one", "o1ne");    
        replaced_string = replaced_string.replace("two", "t2wo");
        replaced_string = replaced_string.replace("three", "thr3e");
        replaced_string = replaced_string.replace("four", "f4ur");
        replaced_string = replaced_string.replace("five", "f5ve");
        replaced_string = replaced_string.replace("six", "s6x");
        replaced_string = replaced_string.replace("seven", "s7ven");
        replaced_string = replaced_string.replace("eight", "e8ight");
        replaced_string = replaced_string.replace("nine", "n9ne");

        let mut first_digit = true;
        let mut number_str = String::new();    
        let mut last_digit_char = '0';
        for c in replaced_string.chars() {
                if c.is_digit(10)
                {
                    if first_digit
                    {
                        first_digit = false;
                        number_str.push(c);
                    }
                    last_digit_char = c;
                }
        }
        number_str.push(last_digit_char);
        parsed_vec.push(number_str.parse::<i32>().unwrap());
    }

    let mut sum = 0;
    for number in parsed_vec.iter()
    {
        println!("{}", number);
        sum += number;
    }

    println!("sum:{}", sum);

}
