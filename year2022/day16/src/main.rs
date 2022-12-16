use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut valves: Vec<Valve> = Vec::new();
    while let Some(line) = lines.next() {
        let mut new_valve = Valve {
            id: "XX".to_string(),
            flow_rate: 0,
            tunnels: Vec::new(),
        };
        let mut middle_split: Vec<&str> = line.split(";").collect();

        let mut first_words: Vec<&str> = middle_split[0].split(" ").collect();
        new_valve.id = first_words[1].to_string();
        let flow_rate_string: Vec<&str> = first_words[4].split("rate=").collect();
        new_valve.flow_rate = flow_rate_string[1].parse::<i32>().unwrap();

        let mut tunnels_big_string: Vec<&str> =
            middle_split[1].split("tunnels lead to valves ").collect();
        let mut remove_whitespaces: Vec<&str> = tunnels_big_string[1].split(" ").collect();
        for word in remove_whitespaces {
            new_valve.tunnels.push(word.to_string().replace(",", ""));
        }

        valves.push(new_valve);
    }

    let mut paths_from_tunnel: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for valve in &valves {
        paths_from_tunnel.insert(valve.id.clone(), vec![(valve.id.clone(), 0); 1]);
    }
    for valve in &valves {
        return_all_tunnels_from_tunnel(
            valve.id.clone(),
            &valves,
            &mut paths_from_tunnel,
            valve.id.clone(),
            1,
        );
    }

    let mut open_valves: Vec<String> = Vec::new();
    let mut cooldown = 30;
    let mut pressure_released = 0;
    let mut current_place: String = "AA".to_string();
    while cooldown > 1 {
        let mut best_value = 0;
        let mut cooldown_of_path = 1;
        let mut best_valve_name = "".to_string();
        for path in &paths_from_tunnel[&current_place] {
            let value_of_path =
                (cooldown - path.1 - 1) * find_valve(&valves, path.0.clone()).unwrap().flow_rate;
            if !open_valves.contains(&path.0) && value_of_path > best_value && cooldown > path.1 {
                best_value = value_of_path;
                cooldown_of_path = path.1 + 1;
                best_valve_name = path.0.clone();
            }
        }
        if best_value > 0 {
            println!(
                "moving from:{}, to open: {}, this will release: {}, time lefT: {}",
                current_place, best_valve_name, best_value, cooldown
            );
        }
        cooldown -= cooldown_of_path;
        pressure_released += best_value;
        if best_valve_name != "" {
            open_valves.push(best_valve_name.clone());
            current_place = best_valve_name.clone();
        }
    }

    println!("Pressure released: {}", pressure_released);
}

fn find_valve(valves: &Vec<Valve>, id: String) -> Option<Valve> {
    for valve in valves {
        if valve.id == id {
            return Some(valve.clone());
        }
    }

    None
}

fn return_all_tunnels_from_tunnel(
    start_tunnel: String,
    valves: &Vec<Valve>,
    mut path_from_tunnel_map: &mut HashMap<String, Vec<(String, i32)>>,
    tunnel_to_check: String,
    depth: i32,
) {
    for i in 0..valves.len() {
        let valve = &valves[i];
        if valve.id == tunnel_to_check {
            for tunnel_from_here in &valve.tunnels {
                let mut add_tunnel: bool = true;
                for q in 0..path_from_tunnel_map[&start_tunnel].len() {
                    if *tunnel_from_here == path_from_tunnel_map[&start_tunnel][q].0 {
                        if depth < path_from_tunnel_map[&start_tunnel][q].1 {
                            path_from_tunnel_map.get_mut(&start_tunnel).unwrap()[q].1 = depth;
                        }
                        add_tunnel = false;
                    }
                }

                if add_tunnel {
                    path_from_tunnel_map
                        .get_mut(&start_tunnel)
                        .unwrap()
                        .push((tunnel_from_here.clone(), depth));

                    return_all_tunnels_from_tunnel(
                        start_tunnel.clone(),
                        valves,
                        &mut path_from_tunnel_map,
                        tunnel_from_here.clone(),
                        depth + 1,
                    );
                }
            }
        }
    }
}
