#[derive(Clone)]
struct LanternFish {
    day_to_give_birth: i32,
}

fn main() {
    let input = include_str!("day6input.txt");
    let mut parts = input.split(",").map(|s| s.parse::<i32>().unwrap());

    let mut fishies: Vec<LanternFish> = Vec::new();

    while let Some(i) = parts.next() {
        fishies.push(LanternFish {
            day_to_give_birth: i,
        });
    }
    let mut new_fishies: Vec<LanternFish> = Vec::new();
    //TASK 1
    let mut fishies_clone = fishies.clone();
    for _ in 0..80 {
        for mut fish in fishies_clone.iter_mut() {
            if fish.day_to_give_birth == 0 {
                new_fishies.push(LanternFish {
                    day_to_give_birth: 8,
                });
                fish.day_to_give_birth = 6;
            } else {
                fish.day_to_give_birth -= 1;
            }
        }

        for fish in new_fishies.iter() {
            fishies_clone.push(fish.clone());
        }
        new_fishies.clear();
    }
    println!("fishies_clone: {}", fishies_clone.len());
    // 0, 1 , 2, 3 , 4, 5 , 6 , 7 ,8
    //TASK 2
    let len = 9;

    let fishies_clone = fishies.clone();

    let mut generation: Vec<i64> = vec![0; 9];

    for fish in fishies_clone.iter() {
        generation[(fish.day_to_give_birth + 1) as usize] += 1;
    }
    for i in 0..257 {
        let number_of_births_today = generation[i % len];
        generation[i % len] -= number_of_births_today;
        generation[(i + 7) % len] += number_of_births_today;
        generation[(i + 9) % len] += number_of_births_today;
    }
    let mut sum = 0;
    for day in generation.iter() {
        sum += day;
    }

    println!("All fishies: {}!", sum);
}
