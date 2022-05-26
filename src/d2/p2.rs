pub fn exec(src: String) {

    let mut aim_val: i32 = 0;

    let mut temp_forward: u32;
    let mut forward_val: u32 = 0;

    let mut depth_val: i32 = 0;

    for move_str in src.lines() {
        if move_str.contains("forward") {
            temp_forward = move_str.replace("forward ", "").parse::<u32>().unwrap();
            forward_val += temp_forward;
            depth_val += aim_val * ( temp_forward as i32 );

        } else if move_str.contains("up") {
            aim_val -= move_str.replace("up ", "").parse::<i32>().unwrap();
        } else if move_str.contains("down") {
            aim_val += move_str.replace("down ", "").parse::<i32>().unwrap();
        }
    }

    println!("result: {:?}", (forward_val as i32)*depth_val);
}