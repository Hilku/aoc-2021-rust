fn main() {
    let input = include_str!("day7input.txt");
    let mut parts = input.split(",").map(|s| s.parse::<i32>().unwrap());

    let mut crab_fuels: Vec<i32> = Vec::new();
    while let Some(i) = parts.next() {
        crab_fuels.push(i);
    }

    let mut smollest_number: i32 = 999999;
    let mut highest_number: i32 = 0;

    for fuel in crab_fuels.iter() {
        if *fuel < smollest_number {
            smollest_number = *fuel;
        }
        if *fuel > highest_number {
            highest_number = *fuel;
        }
    }
    println!(
        "Smollest number: {}, Highest Number:{}",
        smollest_number, highest_number
    );
    //TAsk 1
    let mut least_fuel = 99999999;
    let mut iteration = 0;
    for i in smollest_number..highest_number {
        let mut sum = 0;

        for fuel in crab_fuels.iter() {
            sum += (fuel - i).abs();
        }

        if least_fuel > sum {
            least_fuel = sum;
            iteration = i;
        }
    }

    println!(
        "Task 1, 
        Least fuel you can use: {}. The goal number is: {}",
        least_fuel, iteration
    );

    //TASK 2

    let mut least_fuel = 99999999;
    let mut iteration = 0;
    for i in smollest_number..highest_number {
        let mut sum = 0;

        for fuel in crab_fuels.iter() {
            for q in 1..=(fuel - i).abs() {
                sum += q;
            }
        }

        if least_fuel > sum {
            least_fuel = sum;
            iteration = i;
        }
    }

    println!(
        "Task 2,
        Least fuel you can use: {}. The goal number is: {}",
        least_fuel, iteration
    );
}
