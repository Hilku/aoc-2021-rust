#[derive(Debug, Clone, PartialEq)]
pub enum Tiles {
    CucumberEast,
    CucumberSouth,
    Empty,
}

fn main() {
    let input = include_str!("day25input.txt");

    let mut lines = input.lines();
    let mut tiles: Vec<Vec<Tiles>> = Vec::new();

    while let Some(line) = lines.next() {
        tiles.push(Vec::new());
        for c in line.chars() {
            match c {
                '>' => {
                    tiles.last_mut().unwrap().push(Tiles::CucumberEast);
                }
                'v' => {
                    tiles.last_mut().unwrap().push(Tiles::CucumberSouth);
                }
                '.' => {
                    tiles.last_mut().unwrap().push(Tiles::Empty);
                }
                _ => {
                    println!("wrong input");
                }
            }
        }
    }

    let mut moved_this_round = true;
    let mut step_number = 0;

    while moved_this_round {
        step_number += 1;
        moved_this_round = false;
        let mut buffer = tiles.clone();
        //East cucumbers move
        for i in 0..tiles.len(){
            for q in 0..tiles[i].len(){
                match tiles[i][q]
                {
                    Tiles::CucumberEast => {
                        if q < tiles[i].len() - 1 {
                            if tiles[i][q + 1] == Tiles::Empty {
                                buffer[i][q+1] = Tiles::CucumberEast;
                                buffer[i][q] = Tiles::Empty;moved_this_round = true;
                            }
                        } else {
                            if tiles[i][0] == Tiles::Empty {
                                buffer[i][0] = Tiles::CucumberEast;
                                buffer[i][q] = Tiles::Empty;moved_this_round = true;
                            }
                        }
                    }
                    _ => {}
                }                
            }
        }
        tiles = buffer.clone();
        //South cucumbers move
        
        for i in 0..tiles.len(){
            for q in 0..tiles[i].len(){
                match tiles[i][q]
                {
                    Tiles::CucumberSouth => {
                        if i < tiles.len() - 1 {
                            if tiles[i + 1][q] == Tiles::Empty {
                                buffer[i + 1][q] = Tiles::CucumberSouth;
                                buffer[i][q] = Tiles::Empty;moved_this_round = true;
                            }
                        } else {
                            if tiles[0][q] == Tiles::Empty {
                                buffer[0][q] = Tiles::CucumberSouth;
                                buffer[i][q] = Tiles::Empty;moved_this_round = true;
                            }
                        }
                    }
                    _ => {}
                }                
            }
        }
        tiles = buffer.clone();
    }

    println!("Step number: {}", step_number);
}
