use std::collections::{ HashMap, HashSet };

pub fn exec(src: &str, print: bool) {

    let line = src.lines().next().unwrap();

    let stripped = line.strip_prefix("target area: ").unwrap();
    let mut x_and_y = stripped.split(", ");

    let x_define = x_and_y.next().unwrap();
    let y_define = x_and_y.next().unwrap();

    let mut x_vals = x_define.strip_prefix("x=").unwrap().split("..");
    let mut y_vals = y_define.strip_prefix("y=").unwrap().split("..");

    let x_range: (i64, i64) = ( x_vals.next().unwrap().parse::<i64>().unwrap(), x_vals.next().unwrap().parse::<i64>().unwrap() );
    let y_range: (i64, i64) = ( y_vals.next().unwrap().parse::<i64>().unwrap(), y_vals.next().unwrap().parse::<i64>().unwrap() );

    // <x initial-velocity, ( (steps to reach), (intersect-point), (is_static) )>
    let mut x_vel_to_intersect_list: HashMap<i64, Vec<(u64, i64, bool)>> = HashMap::new();

    // find the number of x initial velocities that will reach the target area in multiple steps
    for x_initial in 0..x_range.0 {
        let mut cur_x_pos: i64 = 0;

        let x_init_step_size: i64 = x_initial;

        // iter by number of steps where x-pos can change
        for step_num in 0..x_initial {
            cur_x_pos += x_init_step_size - step_num;

            if cur_x_pos >= x_range.0 && cur_x_pos <= x_range.1 {

                // will x remain in this column for all remaining steps?
                let is_x_static: bool = (x_init_step_size - step_num) <= 1;

                x_vel_to_intersect_list.entry(x_initial).or_insert(vec![]).push(( (step_num+1) as u64, cur_x_pos, is_x_static));
            }
        }
    }

    // INSIGHT: For any static x-intersects (where x is no longer moving):
    //     the number of steps for the y-intersect to happen MUST be >= than the number of steps to reach the x-intersect
    //     Otherwise, could include y-velocities that intersect at one point but have overshot by the time the last x-value is reached
    // For non-last x-intersects, check for y-intersects with equivalent steps

    let mut distinct_y: u64 = 0;

    for ( _x_init, x_intersect_list ) in x_vel_to_intersect_list.iter() {
        let mut found_y_vals: HashSet<i64> = HashSet::new();

        
        for (step_num, _intersect_pt, is_static) in x_intersect_list.iter() {

            let mut y_start: i64 = y_range.0;

            while y_start < y_range.0.abs() {

                let mut cur_step_num: u64 = 0;

                // probe starting y-coord
                let mut y_coord: i64 = 0;
                
                let mut y_test_idx: i64 = 0;
                
                // iterate through steps and check each y-position
                loop {

                    cur_step_num += 1;
                    y_coord += y_test_idx + y_start;

                    // verify y has intersected target area while x has also intersected target area

                    // Note: different conditionals represent cases where x is changing
                    // and where x is no longer changing (all subsequent steps will have x within the target area)
                    if y_coord >= y_range.0 && y_coord <= y_range.1 {
                        if *is_static && cur_step_num >= *step_num ||
                            (!is_static && cur_step_num == *step_num) 
                        {
                            found_y_vals.insert(y_start);
                            break;
                        }
                    }
        
                    // have skipped target area
                    else if y_coord < y_range.0 {
                        break;
                    }
                    y_test_idx -= 1;
                }
                y_start += 1;
            }
        }
        distinct_y += found_y_vals.len() as u64;
    }

    // Calculate number of 1-step velocity pairs, e.g. pairs that directly move to a cell of target area in one step
    // this is equal to number of cells in target area
    distinct_y += ( ((y_range.0 - y_range.1).abs() + 1) * ((x_range.0 - x_range.1).abs() + 1) ) as u64;
    if print { println!("result: {}", distinct_y) }
}