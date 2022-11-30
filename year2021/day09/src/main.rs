fn main() {
    let input = include_str!("day9input.txt");
    let mut parts = input.lines();

    //let mut smoke: Vec<Vec<i32>> = vec![vec![-1; 10]; 5];
    let mut smoke: Vec<Vec<i32>> = vec![vec![-1; 100]; 100];

    let mut current_read_index = 0;
    let mut risk_sum: i32 = 0;
    while let Some(line) = parts.next() {
        let mut smoke_index = 0;
        for c in line.chars() {
            smoke[current_read_index][smoke_index] = c.to_digit(10).unwrap() as i32;
            smoke_index += 1;
        }
        current_read_index += 1;
    }
    let smoke_data = check_smokes(&smoke);
    risk_sum += smoke_data.0;

    println!("Part 1: {}", risk_sum);

    let bussin_basin = check_basins(&smoke, smoke_data.1);

    println!("Part2: {}", bussin_basin);
}

fn check_basins(smoke_data: &Vec<Vec<i32>>, low_points: Vec<(usize, usize)>) -> usize {
    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut occupied_spaces: Vec<(usize, usize)> = Vec::new();
    for low_point in low_points.iter() {
        let i = low_point.0;
        let j = low_point.1;
        let basin_size = check_neighbours(&smoke_data, (i, j), &mut occupied_spaces);
        basin_sizes.push(basin_size as usize);
    }
    basin_sizes.sort();
    println!("Basin sizeS: {:?}", basin_sizes);
    let mut final_size_multiplied = 1;
    for size in basin_sizes.iter().rev().take(3) {
        println!("Final numba: {}", size);
        final_size_multiplied *= size;
    }

    return final_size_multiplied;
}

fn check_neighbours(
    smoke_data: &Vec<Vec<i32>>,
    position: (usize, usize),
    mut occupied: &mut Vec<(usize, usize)>,
) -> usize {
    let x = position.0;
    let y = position.1;
    let current_number = smoke_data[x][y];

    if occupied.contains(&(x, y)) {
        return 0;
    } else {
        occupied.push((x, y))
    }

    if current_number == 9 {
        return 0;
    }
    let mut sum = 1;
    if x < smoke_data.len() - 1 {
        if smoke_data[x + 1][y] > current_number {
            sum += check_neighbours(&smoke_data, (x + 1, y), &mut occupied);
        }
    }
    if x > 0 {
        if smoke_data[x - 1][y] > current_number {
            sum += check_neighbours(&smoke_data, (x - 1, y), &mut occupied);
        }
    }
    if y < smoke_data[x].len() - 1 {
        if smoke_data[x][y + 1] > current_number {
            sum += check_neighbours(&smoke_data, (x, y + 1), &mut occupied);
        }
    }
    if y > 0 {
        if smoke_data[x][y - 1] > current_number {
            sum += check_neighbours(&smoke_data, (x, y - 1), &mut occupied);
        }
    }

    return sum;
}

fn check_smokes(smoke_data: &Vec<Vec<i32>>) -> (i32, Vec<(usize, usize)>) {
    let mut risk_level_sum: i32 = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for i in 0..smoke_data.len() {
        for j in 0..smoke_data[i].len() {
            let mut is_low_point: bool = true;
            //islowpooint
            if i > 0 {
                is_low_point = is_low_point && smoke_data[i][j] < smoke_data[i - 1][j];
            }
            if i < smoke_data.len() - 1 {
                is_low_point = is_low_point && smoke_data[i][j] < smoke_data[i + 1][j];
            }
            if j > 0 {
                is_low_point = is_low_point && smoke_data[i][j] < smoke_data[i][j - 1];
            }
            if j < smoke_data[i].len() - 1 {
                is_low_point = is_low_point && smoke_data[i][j] < smoke_data[i][j + 1];
            }

            if is_low_point {
                low_points.push((i, j));
                risk_level_sum += smoke_data[i][j] + 1
            }
        }
    }

    return (risk_level_sum, low_points);
}
