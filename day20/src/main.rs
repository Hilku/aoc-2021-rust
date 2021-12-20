fn main() {
    let image_input = include_str!("day20imageinput.txt");
    let enhancer_input = include_str!("day20enhancerinput.txt");

    let image_input = include_str!("day20realimageinput.txt");
    let enhancer_input = include_str!("day20bigenhancerinput.txt");

    let mut enhance = Vec::new();

    for c in enhancer_input.chars() {
        if c == '#' || c == '.' {
            enhance.push(c);
        }
    }

    let mut image = vec![];
    let mut image_lines = image_input.lines();
    while let Some(line) = image_lines.next() {
        image.push(Vec::new());
        for c in line.chars() {
            image.last_mut().unwrap().push(c);
        }
    }

    let mut new_image = image.clone();

    for _ in 0..200 {
        for row in new_image.iter_mut() {
            row.insert(0, '.');

            row.push('.');
        }
        new_image.insert(0, vec!['.'; new_image[0].len()]);
        new_image.push(vec!['.'; new_image[0].len()]);
    }
    for step in 0..50 {
        println!("Step: {}", step);
        let mut enhanced_image = new_image.clone();
        //This means its not on the side. (Hopefully)
        for row in 1..new_image.len() - 1 {
            for column in 1..new_image[row].len() - 1 {
                let mut enhance_number = "".to_string();
                enhance_number.push(new_image[row - 1][column - 1]);
                enhance_number.push(new_image[row - 1][column]);
                enhance_number.push(new_image[row - 1][column + 1]);
                enhance_number.push(new_image[row][column - 1]);
                enhance_number.push(new_image[row][column]);
                enhance_number.push(new_image[row][column + 1]);
                enhance_number.push(new_image[row + 1][column - 1]);
                enhance_number.push(new_image[row + 1][column]);
                enhance_number.push(new_image[row + 1][column + 1]);
                enhance_number = enhance_number.replace(".", "0");
                enhance_number = enhance_number.replace("#", "1");
                let enhance_index = usize::from_str_radix(&enhance_number, 2).unwrap();

                enhanced_image[row][column] = enhance[enhance_index];
            }
        }
        new_image = enhanced_image.clone();
    }
    let mut sum = 0;

    for row in 100..new_image.len() - 100 {
        let mut r = "".to_string();
        for column in 100..new_image[row].len() - 100 {
            if new_image[column][row] == '#' {
                sum += 1;
            }
            r.push(new_image[column][row]);
        }
        println!("{}", r);
    }

    println!("Sum:{}", sum);
}
