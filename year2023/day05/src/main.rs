#[derive(Debug, Copy, Clone)]
struct Row {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

fn main() {
    let seeds_input = include_str!("seedmap.txt");
    let seed_to_soil_input = include_str!("seed-to-soil.txt");
    let s_to_f_input = include_str!("soil-to-fertilizer.txt");
    let f_to_w_input = include_str!("fertilizer-to-water.txt");
    let w_to_l_input = include_str!("water-to-light.txt");
    let l_to_t_input = include_str!("light-to-temperature.txt");
    let t_to_h_input = include_str!("temperature-to-humidity.txt");
    let h_to_l_input = include_str!("humidity-to-location.txt");

    let mut seeds: Vec<i64> = Vec::new();
    let mut seed_to_soil: Vec<Row> = Vec::new();
    let mut soil_to_fertilizer: Vec<Row> = Vec::new();
    let mut fertilizer_to_water: Vec<Row> = Vec::new();
    let mut water_to_light: Vec<Row> = Vec::new();
    let mut light_to_temperature: Vec<Row> = Vec::new();
    let mut temperature_to_humidity: Vec<Row> = Vec::new();
    let mut humidity_to_location: Vec<Row> = Vec::new();

    let mut split_stuff = seeds_input.lines();
    while let Some(string) = split_stuff.next() {
        let mut parts = string.split_whitespace();

        while let Some(part) = parts.next() {
            seeds.push(part.parse::<i64>().unwrap());
        }
    }
    let mut seed_ranges: Vec<Row> = Vec::new();
    split_stuff = seeds_input.lines();
    while let Some(string) = split_stuff.next() {
        let mut parts = string.split_whitespace();
        let mut row: Row = Row {
            destination_range_start: 0,
            source_range_start: 0,
            range_length: 0,
        };
        let mut index = 0;
        while let Some(part) = parts.next() {
            index += 1;
            if index >= 2 {
                index = 0;
                row.range_length = part.parse::<i64>().unwrap();
                seed_ranges.push(row);
            } else {
                row.source_range_start = part.parse::<i64>().unwrap();
                row.destination_range_start = part.parse::<i64>().unwrap();
            }
        }
    }
    parse_input(&mut seed_to_soil, seed_to_soil_input);
    parse_input(&mut soil_to_fertilizer, s_to_f_input);
    parse_input(&mut fertilizer_to_water, f_to_w_input);
    parse_input(&mut water_to_light, w_to_l_input);
    parse_input(&mut light_to_temperature, l_to_t_input);
    parse_input(&mut temperature_to_humidity, t_to_h_input);
    parse_input(&mut humidity_to_location, h_to_l_input);

    let mut vector_of_vectors: Vec<Vec<Row>> = Vec::new();
    vector_of_vectors.push(seed_to_soil);
    vector_of_vectors.push(soil_to_fertilizer);
    vector_of_vectors.push(fertilizer_to_water);
    vector_of_vectors.push(water_to_light);
    vector_of_vectors.push(light_to_temperature);
    vector_of_vectors.push(temperature_to_humidity);
    vector_of_vectors.push(humidity_to_location);

    let mut lowest_returned_number: i64 = 999999999;
    for seed_number in seeds {
        let mut returned_number = seed_number;
        for vector in &vector_of_vectors {
            returned_number = get_corresponding_number(returned_number, &vector);
        }

        if returned_number < lowest_returned_number {
            lowest_returned_number = returned_number;
        }
    }
    /*find lowest destination number in map, find the lowest possible number that corresponds to that destination */

    println!("Lowest returned membeR: {}", lowest_returned_number);

    let mut lowest_possible_number: i64 = 325625103999999;

    for seed in seed_ranges {
        for seed_number in seed.source_range_start..seed.source_range_start + seed.range_length {
            let mut returned_number = seed_number;
            for vector in &vector_of_vectors {
                returned_number = get_corresponding_number(returned_number, &vector);
            }

            if returned_number < lowest_possible_number {
                println!("Found lowest number for seed: {}", seed_number);
                lowest_possible_number = returned_number;
                println!("Lowest possible membeR: {}", lowest_possible_number);
            }
        }
        println!(
            "Range done! Lowest number for seed: {}",
            lowest_possible_number
        );
    }
    println!(
        "Finished! Lowest possible member: {}",
        lowest_possible_number
    );
}

fn get_corresponding_number(initial_number: i64, rows_to_check: &Vec<Row>) -> i64 {
    let mut solution_number = initial_number;
    for row in rows_to_check {
        if initial_number >= row.source_range_start
            && initial_number < row.source_range_start + row.range_length
        {
            solution_number =
                row.destination_range_start + (initial_number - row.source_range_start);

            return solution_number;
        }
    }

    return solution_number;
}

fn parse_input(vector_to_fill: &mut Vec<Row>, string_to_parse: &'static str) {
    let mut split_stuff = string_to_parse.lines();
    while let Some(string) = split_stuff.next() {
        let parts: Vec<&str> = string.split_whitespace().collect();
        let mut row: Row = Row {
            destination_range_start: 0,
            source_range_start: 0,
            range_length: 0,
        };
        row.destination_range_start = parts[0].parse::<i64>().unwrap();
        row.source_range_start = parts[1].parse::<i64>().unwrap();
        row.range_length = parts[2].parse::<i64>().unwrap();

        vector_to_fill.push(row);
    }
}
