use crate::util::file;
use std::collections::VecDeque;

const DAYS_TO_SIMULATE: i32 = 80;

pub fn exec() {
    let src: String = file::read_file_arg();

    // (timer, days_remaining)
    let mut fish_queue: VecDeque<(i32, i32)> = VecDeque::new();

    for line in src.lines() {
        for num_str in line.split(',') {
            fish_queue.push_back((num_str.parse().unwrap(), DAYS_TO_SIMULATE));
        }
    }

    let mut cur_fish: (i32, i32);

    let mut timer: i32;
    let mut days_remaining: i32;

    let mut fish_to_spawn: i32;

    let mut total_fish: u64 = 0;

    while !fish_queue.is_empty() {
        cur_fish = fish_queue.pop_front().unwrap();

        timer = cur_fish.0;
        days_remaining = cur_fish.1;

        // if enough time left to spawn more fish
        if (days_remaining - (timer+1)) >= 0 {

            // spawn initial fish

            days_remaining -= timer+1;
            fish_queue.push_back((8, days_remaining));

            // use int division to spawn remaining
            fish_to_spawn = days_remaining / 7;
            for i in 0..fish_to_spawn {
                fish_queue.push_back((8, days_remaining - (i+1)*7));
            }
        }
        total_fish += 1;
    }

    println!("total_fish: {}", total_fish);


}