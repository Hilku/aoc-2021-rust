fn main() {    
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<i32> = Vec::new();
    
    let mut game_id = 1;    
    let mut sum = 0;
    while let Some(string) = split_stuff.next() {
        let split_games : Vec<&str> = string.split(':').collect();
        println!("split_games: {}", split_games[1]);
        let split_sets : Vec<&str> = split_games[1].split(';').collect();
        let mut possible = true;
        for set in split_sets.iter()
        {        
            println!("set: {}", set);
            let split_cubes: Vec<&str> = set.split_whitespace().collect();

            let mut number = 0;
            for i in 0..split_cubes.len()
            {
                println!("{}", split_cubes[i]);
                if i % 2 == 0
                {
                    number = split_cubes[i].parse::<i32>().unwrap();
                }
                else
                {
                    let without_coma = split_cubes[i].replace(",", "");

                    if without_coma == "green"
                    {
                        if number > 13
                        {
                            possible = false;
                            break;
                        }
                    }
                    else if without_coma == "blue"
                    { 
                        println!("helo number is: {}", number);
                        if number > 14 
                        {
                            possible = false;
                            break;
                        }
                    }
                    else if without_coma == "red"
                    {
                        if number > 12
                        {
                            possible = false;
                            break;
                        }
                    }
                }
            }
        }
        if possible
        {
            println!("this game was possible: {}", game_id);
            sum += game_id;
        }
        game_id += 1;
    }
    println!("sum:{}", sum);

    let mut sum_part2 = 0;

    let mut split_stuff2 = input.lines();
    while let Some(string) = split_stuff2.next() {
        let split_games : Vec<&str> = string.split(':').collect();
        let split_sets : Vec<&str> = split_games[1].split(';').collect();
        let mut highest_green = 1;
        let mut highest_red = 1;
        let mut highest_blue = 1;
        
        for set in split_sets.iter()
        {        
            let split_cubes: Vec<&str> = set.split_whitespace().collect();

            let mut number = 0;
            for i in 0..split_cubes.len()
            {
                if i % 2 == 0
                {
                    number = split_cubes[i].parse::<i32>().unwrap();
                }
                else
                {
                    let without_coma = split_cubes[i].replace(",", "");

                    if without_coma == "green"
                    {
                        if number > highest_green
                        {
                            highest_green = number;
                        }
                    }
                    else if without_coma == "blue"
                    { 
                        
                        if number > highest_blue
                        {
                            highest_blue = number;
                        }
                    }
                    else if without_coma == "red"
                    {
                        if number > highest_red
                        {
                            highest_red = number;
                        }
                    }
                }
            }
        }
        sum_part2 += highest_blue * highest_green * highest_red;
    }

    println!("sum_part2: {}", sum_part2);
}
