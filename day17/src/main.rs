#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Probe {
    pub velocity_x: i32,
    pub velocity_y: i32,
    pub pos: Vec2,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn main() {
    let probe = Probe {
        velocity_x: 0,
        velocity_y: 0,
        pos: Vec2 { x: 0, y: 0 },
    };

    let target_area = (Vec2 { x: 96, y: -144 }, Vec2 { x: 125, y: -98 });
    //let target_area = (Vec2 { x: 20, y: -10 }, Vec2 { x: 30, y: -5 });
    let mut reached_x = false;
    let mut initial_velocity_x = 0;
    while !reached_x {
        initial_velocity_x += 1;
        let mut probe_clone = probe.clone();
        probe_clone.velocity_x = initial_velocity_x;
        while probe_clone.velocity_x > 0 {
            make_step(&mut probe_clone);

            if probe_clone.pos.x >= target_area.0.x && probe_clone.pos.x <= target_area.1.x {
                reached_x = true;
                break;
            }
        }
    }
    println!("Reached x with initial vel: {}", initial_velocity_x);

    let mut initial_velocity_y = initial_velocity_x * 50;
    let mut reached_y = false;

    while !reached_y {
        initial_velocity_y -= 1;
        let mut probe_clone = probe.clone();
        probe_clone.velocity_x = initial_velocity_x;
        probe_clone.velocity_y = initial_velocity_y;
        while probe_clone.pos.y > target_area.0.y {
            make_step(&mut probe_clone);
            if probe_clone.pos.y >= target_area.0.y && probe_clone.pos.y <= target_area.1.y {
                reached_y = true;
                break;
            }
        }
    }
    println!("Reached y with initial vel:{}", initial_velocity_y);

    let mut probe_clone = probe.clone();
    probe_clone.velocity_x = initial_velocity_x;
    probe_clone.velocity_y = initial_velocity_y;
    let mut highest_y = 0;
    while !(probe_clone.pos.y >= target_area.0.y && probe_clone.pos.y <= target_area.1.y) {
        make_step(&mut probe_clone);
        if probe_clone.pos.y > highest_y {
            highest_y = probe_clone.pos.y;
        }
    }
    println!("highest y is : {}", highest_y);

    //PART 2
    let mut too_far = false;
    let mut reached_island = 0;
    for initial_x in initial_velocity_x..target_area.1.x * 20 {
        let mut too_low = false;
        let mut initial_y = initial_velocity_y;
        while !too_low {
            let mut probe = Probe {
                velocity_x: initial_x,
                velocity_y: initial_y,
                pos: Vec2 { x: 0, y: 0 },
            };
            let mut make_s = true;
            let mut nr_of_steps = 0;
            while make_s {
                make_step(&mut probe);

                if probe.pos.y < target_area.0.y {
                    if nr_of_steps == 0 {
                        too_low = true;
                    }
                    make_s = false;
                }
                if probe.pos.x > target_area.1.x {
                    if nr_of_steps == 0 {
                        too_far = true;
                    }
                    make_s = false;
                }
                if (probe.pos.y >= target_area.0.y && probe.pos.y <= target_area.1.y)
                    && (probe.pos.x >= target_area.0.x && probe.pos.x <= target_area.1.x)
                {
                    reached_island += 1;
                    break;
                }
                nr_of_steps += 1;
            }

            initial_y -= 1;
        }
        if too_far {
            break;
        }
    }

    println!("nr of possible solutions: {}", reached_island);
}

fn make_step(probe: &mut Probe) {
    probe.pos.x += probe.velocity_x;
    probe.pos.y += probe.velocity_y;
    probe.velocity_x = (probe.velocity_x - 1).max(0);
    probe.velocity_y = probe.velocity_y - 1;
}
