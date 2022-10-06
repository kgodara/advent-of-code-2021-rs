use std::cmp;

pub fn exec(src: &str, print: bool) {

    let mut pos_list: Vec<u64> = vec![];

    for line in src.lines() {
        for pos in line.split(',') {
            pos_list.push(pos.parse().unwrap());
        }
    }


    let mut sum: u64 = 0;
    for pos in pos_list.iter() {
        sum += pos;
    }

    // .round() is not correct, so solve for floor and ceil
    // then compare results
    let ideal_pos_float: f64 = (sum as f64) / (pos_list.len() as f64);

    let ideal_pos_floor: u64 = ideal_pos_float.floor() as u64;
    let ideal_pos_ceil: u64 = ideal_pos_float.ceil() as u64;

    let mut total_fuel_floor: u64 = 0;
    let mut total_fuel_ceil: u64 = 0;

    // sum of consecutive integers: (len/2)*(seq[0] + seq[len-1])
    for pos in pos_list.iter() {
        let dist_floor: f64 = ( cmp::max(*pos, ideal_pos_floor) - cmp::min(*pos, ideal_pos_floor) ) as f64;
        let result_floor: u64 = ( ( dist_floor / 2.0) * ( 1.0 + dist_floor ) ).trunc() as u64;

        let dist_ceil: f64 = ( cmp::max(*pos, ideal_pos_ceil) - cmp::min(*pos, ideal_pos_ceil) ) as f64;
        let result_ceil: u64 = ( ( dist_ceil / 2.0) * ( 1.0 + dist_ceil ) ).trunc() as u64;

        total_fuel_floor += result_floor;
        total_fuel_ceil += result_ceil;
    }

    if print { println!("result: {}", cmp::min(total_fuel_floor, total_fuel_ceil)) }

}