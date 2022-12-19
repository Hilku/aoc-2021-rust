use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct Robot {
    resource_gathered: Resource,
    resource_needed: HashMap<Resource, i32>,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut blueprints: Vec<Vec<Robot>> = Vec::new();
    while let Some(big_line) = lines.next() {
        let robots: Vec<&str> = big_line.split(".").collect();
        let mut robot_vec: Vec<Robot> = Vec::new();
        for i in 0..robots.len() {
            let whitespace_split: Vec<&str> = robots[i].split_whitespace().collect();
            let mut resource_index = 0;
            let mut new_robot = Robot {
                resource_gathered: Resource::Ore,
                resource_needed: HashMap::new(),
            };
            for word in whitespace_split {
                if let Ok(number) = word.parse::<i32>() {
                    if i == 0 {
                        new_robot.resource_needed.insert(Resource::Ore, number);
                    }
                    if i == 1 {
                        new_robot.resource_needed.insert(Resource::Ore, number);
                        new_robot.resource_gathered = Resource::Clay;
                    }
                    if i == 2 {
                        new_robot.resource_gathered = Resource::Obsidian;
                        if resource_index == 0 {
                            new_robot.resource_needed.insert(Resource::Ore, number);
                        } else {
                            new_robot.resource_needed.insert(Resource::Clay, number);
                        }
                    }
                    if i == 3 {
                        new_robot.resource_gathered = Resource::Geode;
                        if resource_index == 0 {
                            new_robot.resource_needed.insert(Resource::Ore, number);
                        } else {
                            new_robot.resource_needed.insert(Resource::Obsidian, number);
                        }
                    }
                    resource_index += 1;
                }
            }
            robot_vec.push(new_robot);
        }
        blueprints.push(robot_vec);
    }

    println!("blueprints: {:?}", blueprints);

    for blueprint in blueprints {
        let mut resource_per_minute: HashMap<Resource, i32> = HashMap::new();
        resource_per_minute.insert(Resource::Ore, 1);
        resource_per_minute.insert(Resource::Clay, 0);
        resource_per_minute.insert(Resource::Obsidian, 0);
        resource_per_minute.insert(Resource::Geode, 0);

        let mut resource_i_have: HashMap<Resource, i32> = HashMap::new();
        resource_i_have.insert(Resource::Ore, 0);
        resource_i_have.insert(Resource::Clay, 0);
        resource_i_have.insert(Resource::Obsidian, 0);
        resource_i_have.insert(Resource::Geode, 0);

        let mut reversed_blueprint = blueprint.clone();
        reversed_blueprint.reverse();

        for minute in 0..24 {
            let mut new_resource_per_min = resource_per_minute.clone();
            let mut new_resource_i_have = resource_i_have.clone();
            for robot in &reversed_blueprint {
                if robot.resource_gathered == Resource::Ore {
                    continue; // for now
                }
                let mut can_build_robot = true;
                for (k, v) in robot.resource_needed.iter() {
                    if *resource_i_have.get(k).unwrap() > *v {
                        can_build_robot = false;
                        break;
                    }
                }
                if can_build_robot {
                    *new_resource_per_min
                        .get_mut(&robot.resource_gathered)
                        .unwrap() += 1;

                    for (k, v) in robot.resource_needed.iter() {
                        *new_resource_i_have.get_mut(k).unwrap() -= *v;
                    }
                }
            }

            for (k, v) in resource_per_minute.iter() {
                *new_resource_i_have.get_mut(k).unwrap() += *v;
            }

            resource_i_have = new_resource_i_have;
            resource_per_minute = new_resource_per_min;
        }
        println!("{:?}", resource_i_have);
    }
}
