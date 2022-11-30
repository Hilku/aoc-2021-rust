#[derive(Debug, Clone, PartialEq)]
struct Scanner {
    pos: Vec3,
    orientation: Vec3,
    relative_beacons: Vec<Beacon>,
}
#[derive(Debug, Clone, PartialEq)]
struct Beacon {
    pos: Vec3,
}
#[derive(Debug, Clone, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let input = include_str!("day19smollinput.txt");
    let mut parts = input.lines();

    let mut scanners: Vec<Scanner> = Vec::new();

    while let Some(line) = parts.next() {
        if line.contains("--- ") {
            scanners.push(Scanner {
                pos: Vec3 { x: 0, y: 0, z: 0 },
                orientation: Vec3 { x: 0, y: 0, z: 0 },
                relative_beacons: Vec::new(),
            })
        } else if line != "" {
            let beacons_positions = line.split(",");
            let mut index = 0;
            let mut new_vec = Vec3 { x: 0, y: 0, z: 0 };
            for pos in beacons_positions {
                match index {
                    0 => {
                        new_vec.x = pos.parse::<i32>().unwrap();
                    }
                    1 => {
                        new_vec.y = pos.parse::<i32>().unwrap();
                    }
                    2 => {
                        new_vec.z = pos.parse::<i32>().unwrap();
                    }
                    _ => {}
                }
                index += 1;
            }
            scanners
                .last_mut()
                .unwrap()
                .relative_beacons
                .push(Beacon { pos: new_vec })
        }
    }

    for scanner in scanners.iter() {
        let scanner_array = get_relativitiy_array(&scanner);
        for other_scanner in scanners.iter() {
            if scanner != other_scanner {
                let mut matches = 0;
                let other_scanners_array = get_relativitiy_array(&other_scanner);
                for beac in scanner_array.iter() {
                    if other_scanners_array.contains(beac) {
                        println!("I matched with this pos: {:?}", beac);
                        matches += 1;
                    }
                }
                println!("Wow i matched with: {}", matches);
            }
        }
    }

    println!("These are the scanners: {:?}", scanners);
}

fn get_relativitiy_array(scanner: &Scanner) -> Vec<Vec3> {
    let mut relativitiy_array: Vec<Vec3> = Vec::new();
    for beacon in scanner.relative_beacons.iter() {
        for rel_beacon in scanner.relative_beacons.iter() {
            if beacon != rel_beacon {
                relativitiy_array.push(Vec3 {
                    x: beacon.pos.x - rel_beacon.pos.x,
                    y: beacon.pos.y - rel_beacon.pos.y,
                    z: beacon.pos.z - rel_beacon.pos.z,
                });
            }
        }
    }

    return relativitiy_array;
}
