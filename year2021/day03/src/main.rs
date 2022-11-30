fn main() {
    let input = include_str!("day3input.txt");

    let mut split_lines_input = input.lines();

    let mut numbers: Vec<isize> = Vec::new();
    //Input parsing
    while let Some(line) = split_lines_input.next() {
        numbers.push(isize::from_str_radix(line, 2).unwrap());
    }
    let number_of_numbers = numbers.len();
    //TASK 1

    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;

    let number_of_one_bits = count_number_of_one_bits(&numbers);

    for i in 0..12 {
        if number_of_one_bits[i] > (number_of_numbers / 2) as i32 {
            gamma_rate += 1 << i;
            println!("Gamma rate: {}", gamma_rate);
        } else {
            epsilon_rate += 1 << i;
        }
    }

    println!(
        "Task 1 Solution: 
    Gamma rate: {},
    Epsilon rate: {},
    Powerconsumption: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    //TASK2
    let mut oxygen_filtered_numbers: Vec<isize> = numbers.clone();

    let mut co2_filtered_numbers: Vec<isize> = numbers.clone();

    for i in (0..12).rev() {
        if oxygen_filtered_numbers.len() > 1 {
            if count_number_of_one_bits(&oxygen_filtered_numbers)[i] as f32
                >= (oxygen_filtered_numbers.len() as f32 / 2.)
            {
                oxygen_filtered_numbers.retain(|number| (*number as i32 & 1 << i) > 0);
            } else {
                oxygen_filtered_numbers.retain(|number| (*number as i32 & 1 << i) == 0);
            }
        }
        if co2_filtered_numbers.len() > 1 {
            if count_number_of_one_bits(&co2_filtered_numbers)[i] as f32
                >= (co2_filtered_numbers.len() as f32 / 2.)
            {
                co2_filtered_numbers.retain(|number| (*number as i32 & 1 << i) == 0);
            } else {
                co2_filtered_numbers.retain(|number| (*number as i32 & 1 << i) > 0);
            }
        }
    }

    println!(
        "Task 2 solution:
    Oxygen Generator rating: {},
    Co2 scrubber rating: {},
    Life support rating: {}",
        oxygen_filtered_numbers[0],
        co2_filtered_numbers[0],
        oxygen_filtered_numbers[0] * co2_filtered_numbers[0]
    );
}

pub fn count_number_of_one_bits(numbers: &Vec<isize>) -> [i32; 12] {
    let mut number_of_one_bits_on_each_place_value: [i32; 12] = [0; 12];
    let mut comparator: isize = 1;
    for i in 0..12 {
        for number in numbers.iter() {
            if number & comparator > 0 {
                number_of_one_bits_on_each_place_value[i] += 1;
            }
        }
        comparator = comparator << 1;
    }

    return number_of_one_bits_on_each_place_value;
}
