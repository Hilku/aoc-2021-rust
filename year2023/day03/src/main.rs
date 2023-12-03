
use std::collections::HashMap;

#[derive(Debug)]
enum Characters {
    NumberJegy(char),
    Operation(char),
    Nothing,
}
#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x : usize,
    y: usize,
}
#[derive(Eq, Hash, PartialEq)]
struct Gear {
    numbers : Vec<i64>,
}

fn main() {    
    let input = include_str!("day1input.txt");
    let mut split_stuff = input.lines();

    let mut parsed_vec: Vec<Vec<Characters>> = Vec::new();
    let mut gears_map: HashMap<Coordinate, Gear> = HashMap::new();
    while let Some(line) = split_stuff.next() {
       let mut parsed_line : Vec<Characters> = Vec::new();
       
       for c in line.chars()
       {
         if c == '.'
         {
            parsed_line.push(Characters::Nothing);
         }
         else if c.is_digit(10)
         {
            parsed_line.push(Characters::NumberJegy(c));
         }
         else
         {
            parsed_line.push(Characters::Operation(c));
         }
       }

       parsed_vec.push(parsed_line);
    }


    let mut sum = 0;
    for i in 0..parsed_vec.len()
    {
        let mut is_part_number = false;
        let mut gear_coordinate : Option<Coordinate> = None;
        let mut part_number_str : String = "".to_string();
        for q in 0..parsed_vec[i].len()
        {
            match parsed_vec[i][q]
            {
               Characters::Nothing =>{
                    if is_part_number && part_number_str != ""
                    {
                        if gear_coordinate.is_some()
                        {
                            match gears_map.get_mut(&gear_coordinate.clone().unwrap()) {
                                Some(g) => {
                                    g.numbers.push(parse_number(part_number_str.clone()));
                                }
                                None => {
                                    let mut gear = Gear {
                                        numbers : Vec::new(),
                                    };
                                    gear.numbers.push(parse_number(part_number_str.clone()));
                                    gears_map.insert(gear_coordinate.clone().unwrap(), gear);
                                }
                            }   
                        }
                        gear_coordinate =  None;
                        is_part_number = false;
                        sum += parse_number(part_number_str);
                    }
                    part_number_str = "".to_string();
               }
               Characters::Operation(c) =>{                
                if part_number_str != ""
               {
                if c == '*'
                {
                    gear_coordinate = Some(Coordinate{
                        x:i,
                        y: q,
                    });
                }
                    if gear_coordinate.is_some()
                    {
                        match gears_map.get_mut(&gear_coordinate.clone().unwrap()) {
                            Some(g) => {
                                g.numbers.push(parse_number(part_number_str.clone()));
                            }
                            None => {
                                let mut gear = Gear {
                                    numbers : Vec::new(),
                                };
                                gear.numbers.push(parse_number(part_number_str.clone()));
                                gears_map.insert(gear_coordinate.clone().unwrap(), gear);
                            }
                        }   
                    }

                    gear_coordinate =  None;
                    is_part_number = false;
                    sum += parse_number(part_number_str);
                }
                part_number_str = "".to_string();
               }
               Characters::NumberJegy(c) => {
                part_number_str.push(c);

                if q > 0
                {
                    match parsed_vec[i][q - 1]
                    {
                        Characters::Operation(c)=>
                        {
                            is_part_number = true;

                            if c == '*'
                            {
                                gear_coordinate = Some(Coordinate{
                                    x:i,
                                    y: q - 1,
                                });
                            }
                        }
                        _ => {}
                    }
                }
                if q < parsed_vec[i].len() - 1
                {
                    match parsed_vec[i][q + 1]
                    {
                        Characters::Operation(c)=>
                        {
                            is_part_number = true;

                            if c == '*'
                            {
                                gear_coordinate = Some(Coordinate{
                                    x:i,
                                    y: q + 1,
                                });
                            }
                        }

                        _ => {}
                    }
                }

                if i > 0
                {
                    match parsed_vec[i - 1][q]
                    {
                        Characters::Operation(c)=>
                        {
                            if c == '*'
                            {
                                gear_coordinate = Some(Coordinate{
                                    x:i - 1,
                                    y: q,
                                });
                            }
                            is_part_number = true;
                        }
                        _ => {}
                    }
                    if q > 0
                    {
                        match parsed_vec[i - 1][q - 1]
                        {
                            Characters::Operation(c)=>
                            {
                                if c == '*'
                                {
                                    gear_coordinate = Some(Coordinate{
                                        x:i - 1,
                                        y: q - 1,
                                    });
                                }
                                is_part_number = true;
                            }
                            _ => {}
                        }
                    }
                    if q < parsed_vec[i - 1].len() - 1
                    {
                        match parsed_vec[i - 1][q + 1]
                        {
                            Characters::Operation(c)=>
                            {
                                if c == '*'
                                {
                                    gear_coordinate = Some(Coordinate{
                                        x:i - 1,
                                        y: q + 1,
                                    });
                                }
                                is_part_number = true;
                            }
                            _ => {}
                        }
                    }

                }
                if i < parsed_vec.len() - 1
                {
                    match parsed_vec[i + 1][q]
                    {
                        Characters::Operation(c)=>
                        {
                            if c == '*'
                            {
                                gear_coordinate = Some(Coordinate{
                                    x:i + 1,
                                    y: q,
                                });
                            }
                            is_part_number = true;
                        }
                        _ => {}
                    }
                    if q > 0
                    {
                        match parsed_vec[i + 1][q - 1]
                        {
                            Characters::Operation(c)=>
                            {
                                if c == '*'
                                {
                                    gear_coordinate = Some(Coordinate{
                                        x:i + 1,
                                        y: q - 1,
                                    });
                                }
                                is_part_number = true;
                            }
                            _ => {}
                        }
                    }
                    if q < parsed_vec[i + 1].len() - 1
                    {
                        match parsed_vec[i + 1][q + 1]
                        {
                            Characters::Operation(c)=>
                            {                            
                                if c == '*'
                            {
                                gear_coordinate = Some(Coordinate{
                                    x:i + 1,
                                    y: q + 1,
                                });
                            }
                                is_part_number = true;
                            }
                            _ => {}
                        }
                    }
                }
               }

            }
        }

        if is_part_number && part_number_str != ""
        {
            if gear_coordinate.is_some()
            {
                match gears_map.get_mut(&gear_coordinate.clone().unwrap()) {
                    Some(g) => {
                        g.numbers.push(parse_number(part_number_str.clone()));
                    }
                    None => {
                        let mut gear = Gear {
                            numbers : Vec::new(),
                        };
                        gear.numbers.push(parse_number(part_number_str.clone()));
                        gears_map.insert(gear_coordinate.clone().unwrap(), gear);
                    }
                }   
            }

            sum += parse_number(part_number_str);
        }
    }
    println!("sum:{}", sum);

    let mut gear_sum = 0;
    for (_, gear) in &gears_map
    {
        if gear.numbers.len() == 2
        {
            gear_sum += gear.numbers[0] * gear.numbers[1];
        }
    }

    println!("gears sum: {}", gear_sum);
}

fn parse_number(number_string : String) -> i64
{
    return number_string.parse::<i64>().unwrap();
}