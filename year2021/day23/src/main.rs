#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    A,
    B,
    C,
    D,
}

fn main() {
    let input = include_str!("day23.txt");

    let mut parts = input.lines();
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    while let Some(line) = parts.next() {
        tiles.push(Vec::new());
        for c in line.chars() {
            match c {
                '#' => {
                    tiles.last_mut().unwrap().push(Tile::Wall);
                }
                '.' => {
                    tiles.last_mut().unwrap().push(Tile::Empty);
                }
                'A' => {
                    tiles.last_mut().unwrap().push(Tile::A);
                }
                'B' => {
                    tiles.last_mut().unwrap().push(Tile::B);
                }
                'C' => {
                    tiles.last_mut().unwrap().push(Tile::C);
                }
                'D' => {
                    tiles.last_mut().unwrap().push(Tile::D);
                }
                _ => {
                    tiles.last_mut().unwrap().push(Tile::Wall);
                }
            }
        }
    }

    println!("{:?}", tiles);
    let mut energy = 0;
    while !is_winning(&tiles) {}
}

fn is_winning(tiles: &Vec<Vec<Tile>>) -> bool {
    return tiles[2][3] == Tile::A
        && tiles[3][3] == Tile::A
        && tiles[2][5] == Tile::B
        && tiles[3][5] == Tile::B
        && tiles[2][7] == Tile::C
        && tiles[3][7] == Tile::C
        && tiles[2][9] == Tile::D
        && tiles[3][9] == Tile::D;
}
