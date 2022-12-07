use std::collections::HashMap;

fn main() {
    let input = include_str!("day7input.txt");
    let mut lines = input.lines();
    let mut directories_sizes : HashMap<String, usize> = HashMap::new();
    let mut directory_stack : Vec<String> = Vec::new(); 
    let mut current_full_path : String = "".to_string();
    let mut last_dir : String = "".to_string();
    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        let first_part = parts.next().unwrap();
        let second_part = parts.next().unwrap();
        if first_part == "$"
        {
            if second_part == "cd"
            {
                let third_part = parts.next().unwrap();
                if third_part == ".."
                {
                    let popped_char = directory_stack.pop().unwrap();
                    for c in last_dir.chars()
                    {
                        current_full_path.pop();
                    }
                    let last_size =  directories_sizes.get(&popped_char).unwrap_or(&0).clone();
                    if directories_sizes.contains_key(&directory_stack.last().cloned().unwrap())
                    {
                        *directories_sizes.get_mut(&directory_stack.last().cloned().unwrap()).unwrap() += last_size;
                    }
                    else
                    {
                        directories_sizes.insert(directory_stack.last().cloned().unwrap(), last_size);
                    }
                }
                else
                {
                    current_full_path += &third_part.parse::<String>().unwrap();
                    directory_stack.push(current_full_path.clone());
                    last_dir = third_part.parse::<String>().unwrap();
                }
            }
            else if second_part == "ls" || second_part == "dir"
            {

            }
        }
        else if first_part == "dir"
        {
            //skip for now
        }
        else {
           let byte_size = first_part.parse::<usize>().unwrap();
           let key = directory_stack.last().cloned().unwrap();
           if directories_sizes.contains_key(&key)
           {
               *directories_sizes.get_mut(&key).unwrap() += byte_size;
           }
           else
           {
                directories_sizes.insert(key, byte_size);
           }
        } 
    }

    while directory_stack.len() > 1
    {
        let popped_char = directory_stack.pop();
        let last_size =  directories_sizes.get(&popped_char.unwrap()).unwrap().clone();

        if directories_sizes.contains_key(&directory_stack.last().cloned().unwrap())
        {
            *directories_sizes.get_mut(&directory_stack.last().cloned().unwrap()).unwrap() += last_size;
        }
        else
        {
             directories_sizes.insert(directory_stack.last().cloned().unwrap(), last_size);
        }
    }


    let mut sum :usize= 0;
    for (key, value) in &directories_sizes {
        if  value <= &(100000 as usize)
        {
            sum += value;
//            println!("{} : {}", key, value);
        }
    }
    println!("sum: {}", sum);

    let max_space = 70000000;
    let space_needed =  30000000;
    let free_space = max_space - directories_sizes.get("/").unwrap();
    println!("Free space is: {}", free_space);

    let mut smallest = max_space;
    for (_key, value) in &directories_sizes {
        if  free_space + value > space_needed
        {
            if value < &smallest
            {
                smallest = *value;
            }
        }
    }
    println!("smallest to delete: {}", smallest);
}
