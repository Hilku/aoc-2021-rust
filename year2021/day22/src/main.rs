use std::collections::HashMap;
#[derive(Debug, Clone)]
struct Command {
    on: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}
#[derive(Debug, Clone, PartialEq, Copy)]
struct Cube {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}
fn main() {
    let input = include_str!("day22input.txt");

    let mut parts = input.lines();
    let mut commands: Vec<Command> = Vec::new();
    while let Some(line) = parts.next() {
        let mut new_command = Command {
            on: false,
            x: (0, 0),
            y: (0, 0),
            z: (0, 0),
        };
        let mut two_parts = line.split(" ");
        let onoroff = two_parts.next().unwrap();
        if onoroff == "on" {
            new_command.on = true;
        }

        let numbers = two_parts.next().unwrap();

        let mut split_numbers = numbers.split(",");

        while let Some(number) = split_numbers.next() {
            let (coordinate, numbers) = number.split_at(2);

            let mut two_numbers = numbers.split("..");
            let first_nr;
            let second_nr;

            first_nr = two_numbers.next().unwrap().parse::<i64>().unwrap();
            second_nr = two_numbers.next().unwrap().parse::<i64>().unwrap();

            match coordinate.chars().nth(0).unwrap() {
                'x' => {
                    new_command.x.0 = first_nr;
                    new_command.x.1 = second_nr;
                }
                'y' => {
                    new_command.y.0 = first_nr;
                    new_command.y.1 = second_nr;
                }
                'z' => {
                    new_command.z.0 = first_nr;
                    new_command.z.1 = second_nr;
                }
                _ => {
                    println!("Input parsing is doo doo.");
                }
            }
        }
        commands.push(new_command);
    }

    let mut cubes: Vec<Cube> = Vec::new();

    for command in commands.iter() {
        let new_cube = Cube {
            x: command.x,
            y: command.y,
            z: command.z,
        };
        let old_cubes = cubes.clone();
        for cube in old_cubes.iter() {
            if is_overlapping(&cube, &new_cube) {
                cubes.retain(|&x| x != *cube);
                if cube.x.0 < new_cube.x.0 {
                    cubes.push(Cube {
                        x: (cube.x.0, new_cube.x.0 - 1),
                        y: cube.y,
                        z: cube.z,
                    });
                }
                if cube.x.1 > new_cube.x.1 {
                    cubes.push(Cube {
                        x: (new_cube.x.1 + 1, cube.x.1),
                        y: cube.y,
                        z: cube.z,
                    });
                }
                if cube.y.0 < new_cube.y.0 {
                    cubes.push(Cube {
                        x: (cube.x.0.max(new_cube.x.0), cube.x.1.min(new_cube.x.1)),
                        y: (cube.y.0, new_cube.y.0 - 1),
                        z: cube.z,
                    })
                }
                if cube.y.1 > new_cube.y.1 {
                    cubes.push(Cube {
                        x: (cube.x.0.max(new_cube.x.0), cube.x.1.min(new_cube.x.1)),
                        y: (new_cube.y.1 + 1, cube.y.1),
                        z: cube.z,
                    })
                }
                if cube.z.0 < new_cube.z.0 {
                    cubes.push(Cube {
                        x: (cube.x.0.max(new_cube.x.0), cube.x.1.min(new_cube.x.1)),
                        y: (cube.y.0.max(new_cube.y.0), cube.y.1.min(new_cube.y.1)),
                        z: (cube.z.0, new_cube.z.0 - 1),
                    })
                }
                if cube.z.1 > new_cube.z.1 {
                    cubes.push(Cube {
                        x: (cube.x.0.max(new_cube.x.0), cube.x.1.min(new_cube.x.1)),
                        y: (cube.y.0.max(new_cube.y.0), cube.y.1.min(new_cube.y.1)),
                        z: (new_cube.z.1 + 1, cube.z.1),
                    })
                }
            }
        }
        if command.on {
            cubes.push(new_cube);
        }
    }

    println!("Sum is: {}", cubes.len());

    let mut sum: usize = 0;
    for cmd in cubes.iter() {
        sum += (((cmd.x.1 - cmd.x.0).abs() + 1)
            * ((cmd.y.1 - cmd.y.0 + 1).abs() + 1)
            * ((cmd.z.1 - cmd.z.0 + 1).abs() + 1)) as usize;
    }
    println!("Sum is:{}", sum);
}

fn is_overlapping(c1: &Cube, c2: &Cube) -> bool {
    /*return !(c1.x.0 > c2.x.1
    || c1.x.1 < c2.x.0
    || c1.y.0 > c2.y.1
    || c1.y.1 < c2.y.0
    || c1.z.0 > c2.z.1
    || c1.z.1 < c2.z.0);*/

    return c1.x.0 <= c2.x.1
        && c1.x.1 >= c2.x.0
        && c1.y.0 <= c2.y.1
        && c1.y.1 >= c2.y.0
        && c1.z.0 <= c2.z.1
        && c1.z.1 >= c2.z.0;
}
