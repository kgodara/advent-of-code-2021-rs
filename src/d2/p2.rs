pub fn exec(src: &str, print: bool) {

    let mut aim_val: i32 = 0;
    let mut forward_val: i32 = 0;
    let mut depth_val: i32 = 0;

    for move_str in src.lines() {
        let mut move_args = move_str.split_whitespace();
        let dir = move_args.next().unwrap();
        let val = move_args.next().unwrap().parse::<i32>().unwrap();

        if dir == "forward" {
            forward_val += val;
            depth_val += aim_val * val;
        } else if dir == "up" {
            aim_val -= val;
        } else if dir == "down" {
            aim_val += val;
        }
    }

    if print { println!("result: {:?}", forward_val * depth_val) }
}