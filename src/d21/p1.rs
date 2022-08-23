

pub fn exec (src: String) {
    let mut lines = src.lines();

    let mut p1_pos: u16 = str::parse(lines.next().unwrap().split(" starting position: ").last().unwrap()).unwrap();
    let mut p2_pos: u16 = str::parse(lines.next().unwrap().split(" starting position: ").last().unwrap()).unwrap();

    let mut p1_score: u16 = 0;
    let mut p2_score: u16 = 0;

    let mut roll_start_val: u16 = 1;

    // Note: roll_start_val is always represented % 10
    while p1_score < 1000 && p2_score < 1000 {
        // calculate sum of 3 subsequent rolls
        let roll_sum: u16 = roll_start_val + roll_start_val + 1 + roll_start_val + 2;

        if (roll_start_val-1) % 2 == 0 {
            // Perform modulo on range [0-9], then +1 to remove zero-index
            p1_pos = ((p1_pos + roll_sum - 1) % 10)+1;
            p1_score += p1_pos;

        } else {
            // Perform modulo on range [0-9], then +1 to remove zero-index
            p2_pos = ((p2_pos + roll_sum - 1) % 10)+1;
            p2_score += p2_pos;
        }

        roll_start_val += 3;
    }

    let result: u32 = (std::cmp::min(p1_score, p2_score) as u32) * ((roll_start_val-1) as u32);
    println!("result: {}", result);
}