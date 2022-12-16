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
        paths_from_tunnel.insert(valve.id.clone(), Vec::new());
    }

    println!("{:?}", valves);
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
            let current_known_tunnels = path_from_tunnel_map[&start_tunnel].clone();
            for tunnel_from_here in &valve.tunnels {
                let mut add_tunnel: bool = true;
                for q in 0..current_known_tunnels.len() {
                    if *tunnel_from_here == current_known_tunnels[q].0 {
                        add_tunnel = false;
                    }
                }

                if add_tunnel {
                    path_from_tunnel_map
                        .get_mut(&start_tunnel)
                        .unwrap()
                        .push((tunnel_from_here.clone(), depth));
                }
            }
        }
    }
}
