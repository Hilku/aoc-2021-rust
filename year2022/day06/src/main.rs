fn main() {
    solve(4);
    solve(14);
}

fn solve(number_of_chars: usize) {
    let input = include_str!("day6input.txt");
    let mut lines = input.lines();
    let mut start_index = 0;

    while let Some(line) = lines.next() {
        unsafe {
            while start_index + number_of_chars < line.len() {
                let four_chars = line.get_unchecked(start_index..start_index + number_of_chars);

                let mut unique_chars: Vec<char> = Vec::new();
                for c in four_chars.chars() {
                    if !unique_chars.contains(&c) {
                        unique_chars.push(c);
                    }
                }

                if unique_chars.len() == number_of_chars {
                    println!(
                        "Solution for nr of chars {} : {}",
                        number_of_chars,
                        start_index + number_of_chars
                    );
                    break;
                }
                start_index += 1;
            }
        }
    }
}
