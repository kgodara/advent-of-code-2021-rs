use std::cmp;

pub fn exec(src: String) {

    let mut pos_list: Vec<u64> = vec![];

    for line in src.lines() {
        for pos in line.split(',') {
            pos_list.push(pos.parse().unwrap());
        }
    }

    pos_list.sort_unstable();

    let ideal_pos: u64 =
        if pos_list.len() % 2 != 0 {
            pos_list[pos_list.len() / 2]
        } else {
            (( ( pos_list[pos_list.len() / 2] + pos_list[(pos_list.len() / 2) - 1] ) as f64 ) / 2.0 ).round() as u64
        }
    ;

    let mut total_fuel: u64 = 0;

    for pos in pos_list {
        total_fuel += cmp::max(pos, ideal_pos) - cmp::min(pos, ideal_pos);
    }

    println!("result: {}", total_fuel);

}