// TODO: 1. Replace this with iterative approach
// TODO: 2. Replace this with [0; 9] approach based on grouping counts of fish per day

use std::collections::HashMap;

const DAYS_TO_SIMULATE: i32 = 256;

// Assume fish_timer is 8 -- just spawned fish
fn get_spawn_count(mut days_remaining: i64, total_map: &mut HashMap<i64, i64>) -> i64 {

    // can't spawn any more fish
    if days_remaining < 9 {
        1
    } else {

        let original_days: i64 = days_remaining;

        // Memoization
        if total_map.contains_key(&days_remaining) {
            return *total_map.get(&days_remaining).unwrap()
        }


        // now fish_timer = 6
        // -9, so the new fish actually spawns
        days_remaining -= 9;
        
        // include current fish
        let mut total_fish: i64 = 1;
        while days_remaining >= 0 {

            // days_remaining after,
            // since we've already spawned the first fish via -=9
            total_fish += get_spawn_count(days_remaining, total_map);
            days_remaining -= 7;
        }

        total_map.entry(original_days).or_insert(total_fish);

        total_fish

    }
}

pub fn exec(src: String) {

    // (timer, days_remaining)
    let mut fish_list: Vec<i32> = Vec::new();

    for line in src.lines() {
        for num_str in line.split(',') {
            fish_list.push(num_str.parse().unwrap());
        }
    }

    let mut days_fish_map: HashMap<i64, i64> = HashMap::new();

    let mut total_fish: i64 = 0;
    total_fish += fish_list.len() as i64;

    let mut days_remaining: i64;

    for fish_timer in fish_list {
        days_remaining = DAYS_TO_SIMULATE as i64;
        // 1. Move fish_timer to 0 and modify days_remaining

        // Verify at least one fish can be spawned
        if days_remaining >= ((fish_timer+1) as i64) {

            // now timer = 0;
            // should be +1 to account for the day involved with spawning
            days_remaining -= (fish_timer+1) as i64;

            while days_remaining >= 0 {
                total_fish += get_spawn_count(days_remaining, &mut days_fish_map);
                days_remaining -= 7;
            }

        }

    }

    println!("result: {}", total_fish);


}