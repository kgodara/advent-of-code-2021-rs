use crate::util::file;

pub fn exec() {
    let src: String = file::read_file_arg();

    let mut forward_val: u32 = 0;
    let mut depth_val: i32 = 0;

    for move_str in src.lines() {
        if move_str.contains("forward") {
            forward_val += move_str.replace("forward ", "").parse::<u32>().unwrap();
        } else if move_str.contains("up") {
            depth_val -= move_str.replace("up ", "").parse::<i32>().unwrap();
        } else if move_str.contains("down") {
            depth_val += move_str.replace("down ", "").parse::<i32>().unwrap();
        }
    }

    println!("horizontal_pos, depth, result: {:?}, {:?}, {:?}", forward_val, depth_val, (forward_val as i32)*depth_val);
}