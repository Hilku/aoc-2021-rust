fn main() {
    let input = include_str!("day16input.txt");
    let mut parts = input.lines();

    let mut binary: String = "".to_string();

    while let Some(line) = parts.next() {
        for c in line.chars() {
            match c {
                '0' => binary.push_str("0000"),
                '1' => binary.push_str("0001"),
                '2' => binary.push_str("0010"),
                '3' => binary.push_str("0011"),
                '4' => binary.push_str("0100"),
                '5' => binary.push_str("0101"),
                '6' => binary.push_str("0110"),
                '7' => binary.push_str("0111"),
                '8' => binary.push_str("1000"),
                '9' => binary.push_str("1001"),
                'A' => binary.push_str("1010"),
                'B' => binary.push_str("1011"),
                'C' => binary.push_str("1100"),
                'D' => binary.push_str("1101"),
                'E' => binary.push_str("1110"),
                'F' => binary.push_str("1111"),
                _ => {
                    println!("Char not handled!")
                }
            }
        }
    }

    let binary_copy = binary.clone();
    let mut version_sum = 0;
    let resolution = handle_packet(&binary_copy, &mut version_sum);
    println!("Version sum is:{}", version_sum);
    println!("res: {:?}", resolution.1);
}

fn make_operation(values: &Vec<i64>, operation: isize) -> i64 {
    println!("Values: {:?}, operation: {}", values, operation);
    match operation {
        0 => return values.iter().sum(),
        1 => {
            let mut prod = 1;
            for v in values.iter() {
                prod *= v;
            }
            return prod;
        }
        2 => {
            let mut min = i64::MAX;
            for v in values.iter() {
                if min > *v {
                    min = *v;
                }
            }
            return min;
        }
        3 => {
            let mut max = 0;
            for v in values.iter() {
                if max < *v {
                    max = *v;
                }
            }
            return max;
        }
        5 => {
            if values[0] > values[1] {
                return 1;
            } else {
                return 0;
            }
        }
        6 => {
            if values[0] < values[1] {
                return 1;
            } else {
                return 0;
            }
        }
        7 => {
            if values[0] == values[1] {
                return 1;
            } else {
                return 0;
            }
        }
        _ => {
            println!("Not handled operation!!");
            return 0;
        }
    }
}

//->usize: number of bits handled
fn handle_packet(packet: &str, mut version_sum: &mut i32) -> (isize, Option<i64>) {
    //First 3 bits - to decimal, version
    let (version_str, remainder) = packet.split_at(3);

    let version = isize::from_str_radix(&version_str, 2).unwrap();
    *version_sum += version as i32;
    let (type_id_str, remainder) = remainder.split_at(3);
    let type_id = isize::from_str_radix(&type_id_str, 2).unwrap();
    match type_id {
        4 => {
            let literal_in_packet = get_literal_from_packet(&remainder);
            return (literal_in_packet.1 + 6, Some(literal_in_packet.0 as i64));
        }
        _ => {
            //Is operator packet
            let (len, remainder) = remainder.split_at(1);
            if len == "0" {
                let (len, remainder) = remainder.split_at(15);
                let mut sub_packet_len = isize::from_str_radix(&len, 2).unwrap();

                let (mut sub_packets, _) = remainder.split_at(sub_packet_len as usize);
                let mut sum_of_handling = 0;
                let mut values: Vec<i64> = Vec::new();
                while sub_packet_len > 0 {
                    let nr_of_bits = handle_packet(&sub_packets, &mut version_sum);
                    sum_of_handling += nr_of_bits.0;
                    sub_packet_len -= nr_of_bits.0;
                    sub_packets = sub_packets.split_at(nr_of_bits.0 as usize).1;

                    match nr_of_bits.1 {
                        Some(value) => {
                            values.push(value);
                        }
                        None => {}
                    }
                }
                let mut return_value = None;
                if values.len() > 0 {
                    return_value = Some(make_operation(&values, type_id));
                }
                return (sum_of_handling + 6 + 15 + 1, return_value);
            } else if len == "1" {
                let mut sum_of_handling = 0;
                let mut values: Vec<i64> = Vec::new();
                let (len, mut remainder) = remainder.split_at(11);
                let nr_of_subpackets = usize::from_str_radix(&len, 2).unwrap();
                for _ in 0..nr_of_subpackets {
                    let nr_of_bits = handle_packet(&remainder, &mut version_sum);
                    sum_of_handling += nr_of_bits.0;
                    remainder = remainder.split_at(nr_of_bits.0 as usize).1;
                    match nr_of_bits.1 {
                        Some(value) => {
                            values.push(value);
                        }
                        None => {}
                    }
                }
                let mut return_value = None;
                if values.len() > 0 {
                    return_value = Some(make_operation(&values, type_id));
                }
                return (sum_of_handling + 6 + 11 + 1, return_value);
            }
        }
    }

    return (6, None);
}

fn get_literal_from_packet(packet: &str) -> (i64, isize) {
    let mut reached_leading_zero = false;
    let mut expect_leading_nr = true;
    let mut binary_representation = "".to_string();
    let mut number_str = "".to_string();
    let mut len = 0;
    for c in packet.chars() {
        len += 1;
        if expect_leading_nr {
            expect_leading_nr = false;
            if c == '1' {
                reached_leading_zero = false;
            } else if c == '0' {
                reached_leading_zero = true;
            }
        } else {
            number_str.push(c);
            if number_str.len() == 4 {
                binary_representation.push_str(&number_str);
                if reached_leading_zero {
                    break;
                }
                expect_leading_nr = true;
                number_str = "".to_string();
            }
        }
    }
    return (
        isize::from_str_radix(&binary_representation, 2).unwrap() as i64,
        len,
    );
}
