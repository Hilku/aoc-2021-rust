enum Controlls {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let input = include_str!("day2input.txt");

    let mut split_stuff = input.lines();

    let mut commands: Vec<Controlls> = Vec::new();

    while let Some(stuff) = split_stuff.next() {
        let split_line: Vec<&str> = stuff.split_whitespace().collect();
        match split_line[0] {
            "forward" => commands.push(Controlls::Forward(split_line[1].parse().unwrap())),
            "up" => commands.push(Controlls::Up(split_line[1].parse().unwrap())),
            "down" => commands.push(Controlls::Down(split_line[1].parse().unwrap())),
            _ => {}
        }
    }
    //TASK 1
    let mut depth = 0;
    let mut horizontal_position = 0;

    for command in commands.iter() {
        match command {
            Controlls::Forward(amount) => horizontal_position += amount,
            Controlls::Up(amount) => {
                depth -= amount;
            }
            Controlls::Down(amount) => {
                depth += amount;
            }
        }
    }

    println!(
        "TASK1: 
        Depth: {}, HorizontalPosition: {}, Multiplied: {}",
        depth,
        horizontal_position,
        depth * horizontal_position
    );

    //TASK 2
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;

    for command in commands.iter() {
        match command {
            Controlls::Forward(amount) => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            Controlls::Up(amount) => {
                aim -= amount;
            }
            Controlls::Down(amount) => {
                aim += amount;
            }
        }
    }

    println!(
        "TASK2: 
        Depth: {}, HorizontalPosition: {}, Multiplied: {}",
        depth,
        horizontal_position,
        depth * horizontal_position
    );
}
