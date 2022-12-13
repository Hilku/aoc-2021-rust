fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut left_packet: String = "".to_string();
    let mut right_packet: String = "".to_string();

    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    let mut line_index = 0;
    let mut packet_index = 1;
    let mut solution_sum = 0;
    while let Some(line) = lines.next() {
        if line_index == 0 {
            left_packet = line.to_string();
            for c in line.chars() {
                if let Some(number) = c.to_digit(10) {
                    left_numbers.push(number);
                }
            }
        }
        if line_index == 1 {
            right_packet = line.to_string();

            for c in line.chars() {
                if let Some(number) = c.to_digit(10) {
                    right_numbers.push(number);
                }
            }
        }
        if line_index == 2 {
            //Compare

            let mut good_order = left_numbers.len() < right_numbers.len();
            if left_numbers.len() == 0 {
                good_order = true;
                println!("left is zero!");
                if right_numbers.len() == 0 {
                    good_order = left_packet.len() < right_packet.len();
                    println!("zero length packets!: {}", packet_index);
                }
            } else {
                for i in 0..left_numbers.len() {
                    if i < right_numbers.len() {
                        if left_numbers[i] > right_numbers[i] {
                            good_order = false;
                            break;
                        } else if left_numbers[i] < right_numbers[i] {
                            good_order = true;
                        }
                    }
                }
            }
            if good_order {
                println!("Adding indice: {}", packet_index);
                solution_sum += packet_index;
            }
            //Compare
            left_packet = "".to_string();
            right_packet = "".to_string();
            left_numbers = Vec::new();
            right_numbers = Vec::new();
            line_index = 0;
            packet_index += 1;
        } else {
            line_index += 1;
        }
    }

    println!("Solution sum: {}", solution_sum);
}
