use std::process;

use std::collections::VecDeque;
use crate::util::file;

pub fn exec() {
    let src: String = file::read_file_arg();

    let mut input_data: Vec<u16> = Vec::new();
    for num_str in src.split_whitespace() {
        input_data.push(num_str.parse::<u16>().unwrap());
    }

    if input_data.len() < 4 {
        println!("0");
        process::exit(1);
    }

    let mut num_increase: u16 = 0;

    let mut window: VecDeque<u16> = VecDeque::from_iter([0, 0, 0]);
    window[0] = input_data[0];
    window[1] = input_data[1];
    window[2] = input_data[2];

    let mut prev_sum: u16 = window.iter().sum();
    
    for (idx, num) in input_data[3..].iter().enumerate() {
        if idx >= (input_data.len()-3) { break; }

        window.pop_front();
        window.push_back(*num);

        if window.iter().sum::<u16>() > prev_sum {
            num_increase += 1;
        }
        prev_sum = window.iter().sum();
    }

    println!("{:?}", num_increase);
}