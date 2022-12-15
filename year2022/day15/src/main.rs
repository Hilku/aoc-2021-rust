#[derive(Debug)]
struct Sensor {
    sensor_pos: (i64, i64),
    closest_beacon_pos: (i64, i64),
    range: i64,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut sensors: Vec<Sensor> = Vec::new();
    while let Some(line) = lines.next() {
        let mut new_sensor: Sensor = Sensor {
            sensor_pos: (0, 0),
            closest_beacon_pos: (0, 0),
            range: 0,
        };
        let mut sensor_beacon_split: Vec<&str> = line.split(":").collect();
        let mut sensor_remove_stuff: Vec<&str> =
            sensor_beacon_split[0].split("Sensor at ").collect();
        let mut sensor_x_y_split: Vec<&str> = sensor_remove_stuff[1].split(",").collect();

        let mut x_number: Vec<&str> = sensor_x_y_split[0].split("x=").collect();
        let mut y_number: Vec<&str> = sensor_x_y_split[1].split(" y=").collect();
        new_sensor.sensor_pos = (
            x_number[1].parse::<i64>().unwrap(),
            y_number[1].parse::<i64>().unwrap(),
        );

        let mut beacon_remove_stuff: Vec<&str> = sensor_beacon_split[1]
            .split(" closest beacon is at ")
            .collect();
        let mut beacon_x_y_split: Vec<&str> = beacon_remove_stuff[1].split(",").collect();

        let mut beacon_x_number: Vec<&str> = beacon_x_y_split[0].split("x=").collect();
        let mut beacon_y_number: Vec<&str> = beacon_x_y_split[1].split(" y=").collect();
        new_sensor.closest_beacon_pos = (
            beacon_x_number[1].parse::<i64>().unwrap(),
            beacon_y_number[1].parse::<i64>().unwrap(),
        );

        let mut range = 0;
        if new_sensor.sensor_pos.0 > new_sensor.closest_beacon_pos.0 {
            range += new_sensor.sensor_pos.0 - new_sensor.closest_beacon_pos.0;
        } else {
            range += new_sensor.closest_beacon_pos.0 - new_sensor.sensor_pos.0;
        }
        if new_sensor.sensor_pos.1 > new_sensor.closest_beacon_pos.1 {
            range += new_sensor.sensor_pos.1 - new_sensor.closest_beacon_pos.1;
        } else {
            range += new_sensor.closest_beacon_pos.1 - new_sensor.sensor_pos.1;
        }
        new_sensor.range = range;

        sensors.push(new_sensor);
    }
    for y_coordinate_to_check in 0..4000000 {
        let mut solution_x_ranges: Vec<(i64, i64)> = Vec::new();
        for sensor in &sensors {
            let coordinate_to_check = (sensor.sensor_pos.0, y_coordinate_to_check);
            let distance = check_distance(sensor.sensor_pos, coordinate_to_check);
            if distance <= sensor.range {
                let distance_to_go_both_directions = sensor.range - distance;
                let new_range = (
                    sensor.sensor_pos.0 - distance_to_go_both_directions,
                    sensor.sensor_pos.0 + distance_to_go_both_directions,
                );

                solution_x_ranges.push(new_range);
            }
        }

        let mut reduced = true;
        while merge_ranges(&mut solution_x_ranges) {}

        let mut solution = 0;

        solution_x_ranges.sort();
        println!("{:?}", solution_x_ranges);
        for solution_x_range in solution_x_ranges {
            solution += solution_x_range.1 - solution_x_range.0;
        }

        println!("{}", solution);
    }
}

fn check_distance(first_coordinate: (i64, i64), second_coordinate: (i64, i64)) -> i64 {
    let mut range = 0;
    if first_coordinate.0 > second_coordinate.0 {
        range += first_coordinate.0 - second_coordinate.0;
    } else {
        range += second_coordinate.0 - first_coordinate.0;
    }
    if first_coordinate.1 > second_coordinate.1 {
        range += first_coordinate.1 - second_coordinate.1;
    } else {
        range += second_coordinate.1 - first_coordinate.1;
    }

    return range;
}

fn merge_ranges(solution_x_ranges: &mut Vec<(i64, i64)>) -> bool {
    for q in 0..solution_x_ranges.len() {
        let new_range = solution_x_ranges[q];
        let mut range_to_remove: Option<usize> = None;
        let mut newest_range: (i64, i64) = new_range;
        for i in 0..solution_x_ranges.len() {
            let range = solution_x_ranges[i];
            if range.0 == new_range.0 && range.1 == new_range.1 {
                continue;
            }
            //range 14,24, new_range: 14,18
            if range.0 <= new_range.0 && range.1 > new_range.0 {
                range_to_remove = Some(i);
            } else if range.0 >= new_range.0 && range.0 < new_range.1 {
                range_to_remove = Some(i);
            }

            if let Some(to_remove) = range_to_remove {
                if range.0 < new_range.0 {
                    newest_range.0 = range.0;
                } else {
                    newest_range.0 = new_range.0;
                }
                if range.1 > new_range.1 {
                    newest_range.1 = range.1;
                } else {
                    newest_range.1 = new_range.1;
                }
                solution_x_ranges.remove(i);
                solution_x_ranges.remove(q);
                if !solution_x_ranges.contains(&newest_range) {
                    solution_x_ranges.push(newest_range);
                }
                return true;
            }
        }
    }

    return false;
}
