use std::cmp::{min, max};

struct Cuboid {


    x1: i32,
    x2: i32,
    
    y1: i32,
    y2: i32,

    z1: i32,
    z2: i32,

    is_on_cuboid: bool,

    // list of 'on' and 'off' cuboids within parent cuboid
    sub_cuboids: Vec<Cuboid>,
}

fn print_cuboid(c: &Cuboid) {
    println!("({}, {}), ({}, {}), ({}, {})", c.x1, c.x2, c.y1, c.y2, c.z1, c.z2);
}

// Intersection if range 1 is somehow overlapped with range 2 or swallows it
// Note: only care if c_two is in c_one's space
fn has_intersect(c_one: &Cuboid, c_two: &Cuboid) -> bool {
    (c_two.x1 <= c_one.x2 && c_two.x2 >= c_one.x1) &&
    (c_two.y1 <= c_one.y2 && c_two.y2 >= c_one.y1) &&
    (c_two.z1 <= c_one.z2 && c_two.z2 >= c_one.z1)
}

fn cuboid_vol(cuboid: &Cuboid) -> u64 {
    ((cuboid.x2 - cuboid.x1 + 1) *
    (cuboid.y2 - cuboid.y1 + 1) *
    (cuboid.z2 - cuboid.z1 + 1)) as u64
}

pub fn exec(src: String) {
    // general approach:
    // iterate over cuboids and off commands
    //     if cuboid, add to list of seen cuboids
    //     if off cmd, apply to all cuboids in seen cuboids
    let lines = src.lines();

    let mut seen_cuboids: Vec<Cuboid> = vec![];

    // let mut on_set: BTreeSet<(i32, i32, i32)> = BTreeSet::new();


    for (_line_idx, line) in lines.enumerate() {
        let is_on_cmd: bool = line.contains("on");

        let new_line: &str = line.split('x').last().unwrap();

        let mut in_range: bool = false;
        let mut cur_range: String = String::new();
        let mut ranges: Vec<String> = vec![];
        let mut seen_dot: bool = false;

        for ch in new_line.chars() {
            
            if ch == 'y' || ch == 'z' {

                ranges.push(cur_range);

                cur_range = String::default();
                in_range = false;
                seen_dot = false;
            }

            else if char::is_numeric(ch) || ch == '-' {
                in_range = true;
                cur_range.push(ch);
            }
            else if in_range && !seen_dot {
                cur_range.push(ch);
                seen_dot = true;
            }
        }
        ranges.push(cur_range);

        let x_range: Vec<i32> = ranges[0].split('.').map(|coord| str::parse(coord).unwrap()).collect();
        let y_range: Vec<i32> = ranges[1].split('.').map(|coord| str::parse(coord).unwrap()).collect();
        let z_range: Vec<i32> = ranges[2].split('.').map(|coord| str::parse(coord).unwrap()).collect();
        




        // no intersect with init region
        if x_range[0] > 50 || x_range[1] < -50 {
            continue;
        }
        if y_range[0] > 50 || y_range[1] < -50 {
            continue;
        }
        if z_range[0] > 50 || z_range[1] < -50 {
            continue;
        }

        let cur_cuboid: Cuboid = Cuboid { x1: max(x_range[0], -50), x2: min(x_range[1], 50),
            y1: max(y_range[0], -50), y2: min(y_range[1], 50),
            z1: max(z_range[0], -50), z2: min(z_range[1], 50),
            is_on_cuboid: is_on_cmd,

            sub_cuboids: vec![],
        };

        if is_on_cmd {

            // if cuboid intersects cuboid in seen_cuboids:
            //     iterate over cuboid's 'off' cube hashset, remove any intersecting cubes
            for on_cuboid in seen_cuboids.iter_mut() {
                // println!("on cube intersect!");

                // push "on" sub_cuboid for all prev "on" cuboids
                if has_intersect(on_cuboid, &cur_cuboid) {
                    on_cuboid.sub_cuboids.push(Cuboid {
                        x1: max(cur_cuboid.x1, on_cuboid.x1), x2: min(cur_cuboid.x2, on_cuboid.x2),
                        y1: max(cur_cuboid.y1, on_cuboid.y1), y2: min(cur_cuboid.y2, on_cuboid.y2),
                        z1: max(cur_cuboid.z1, on_cuboid.z1), z2: min(cur_cuboid.z2, on_cuboid.z2),

                        is_on_cuboid: true,

                        sub_cuboids: vec![],
                    });
                }
            }

            seen_cuboids.push(cur_cuboid);

        } else {
            // handle off cmd
            for on_cuboid in seen_cuboids.iter_mut() {
                // push "off" sub_cuboid for all prev "on" cuboids
                if has_intersect(on_cuboid, &cur_cuboid) {
                    on_cuboid.sub_cuboids.push(Cuboid {
                        x1: max(cur_cuboid.x1, on_cuboid.x1), x2: min(cur_cuboid.x2, on_cuboid.x2),
                        y1: max(cur_cuboid.y1, on_cuboid.y1), y2: min(cur_cuboid.y2, on_cuboid.y2),
                        z1: max(cur_cuboid.z1, on_cuboid.z1), z2: min(cur_cuboid.z2, on_cuboid.z2),

                        is_on_cuboid: false,

                        sub_cuboids: vec![],
                    });
                }
            }
        }
        // println!("processed #{} cube", line_idx+1);
    }

    /* Final summation process:
        for each "on" cuboid:

            // NOTE: if "on" sub_cuboid has no intersects with "off" OR "on" sub_cuboids, can take vol and subtract at end
            // ignore_cube_set is only used to prevent double-counting with off cubes

            for each "on" sub_cuboid:
                for each cube in cuboid:
                    add to ignore_cube_set

            // NOTE: if "off" sub_cuboid has no intersects with "off" OR "on" sub_cuboids, can take vol and subtract at end
            for each "off" sub_cuboid:
                for each cube in cuboid:
                    if cube not in ignore_cube_set: add to off_cube_set
            vol += cube_vol - ignore_cube_set.len() - ignore_cube_count - off_cube_set.len() - off_cube_count

        Notes: if "on" cuboid: none of these cubes need to be counted at all
            (they're intersects and will be handled by future "on" cuboid)
    */



    let mut num_on_cubes: u64 = 0;
    for (_cuboid_idx, cur_cuboid) in seen_cuboids.iter().enumerate() {

        let mut ignore_cube_count: u64 = 0;
        let mut off_cube_count: u64 = 0;

        let on_subcuboids: Vec<&Cuboid> = cur_cuboid.sub_cuboids.iter().filter(|x| x.is_on_cuboid).collect();
        let off_subcuboids: Vec<&Cuboid> = cur_cuboid.sub_cuboids.iter().filter(|x| !x.is_on_cuboid).collect();

        // enter all cubes from first "on" cuboid (no conflict yet)
        if !on_subcuboids.is_empty() {
            ignore_cube_count += cuboid_vol(on_subcuboids[0]);
        }

        for sub_cuboid_idx in 1..on_subcuboids.len() {
            let cur_on_subcuboid = on_subcuboids[sub_cuboid_idx];

            // Check all 'cur_on_subcuboid' cubes against all prev on_subcuboid ranges

            // Could we try generating the intersect cubes and using those?

            // determine number of unique 'on' cubes from 'on_subcuboids'
            for x_coord in cur_on_subcuboid.x1..(cur_on_subcuboid.x2+1) {
                for y_coord in cur_on_subcuboid.y1..(cur_on_subcuboid.y2+1) {
                    for z_coord in cur_on_subcuboid.z1..(cur_on_subcuboid.z2+1) {

                        let mut found_intersect: bool = false;
                        // check cur coord against all prev cuboid ranges
                        for prev_on_subcuboid in on_subcuboids[0..sub_cuboid_idx].iter() {
                            // if intersect
                            if
                            (x_coord >= prev_on_subcuboid.x1 && x_coord <= prev_on_subcuboid.x2) &&
                            (y_coord >= prev_on_subcuboid.y1 && y_coord <= prev_on_subcuboid.y2) &&
                            (z_coord >= prev_on_subcuboid.z1 && z_coord <= prev_on_subcuboid.z2)
                            {
                                found_intersect = true;
                                break;
                            }
                        }

                        if !found_intersect {
                            ignore_cube_count += 1;
                        }
                    }
                }
            }
        }

        // println!("ignore_cube_set.len(): {}", ignore_cube_set.len());

        // enter all cubes from first "off" cuboid (no conflict yet)
        // if !off_subcuboids.is_empty() {
        //     off_cube_count += cuboid_vol(off_subcuboids[0]);
        // }

        for sub_cuboid_idx in 0..off_subcuboids.len() {
            let cur_off_subcuboid = off_subcuboids[sub_cuboid_idx];

            // Check all 'cur_off_subcuboid' cubes against all prev off_subcuboid ranges
            for x_coord in cur_off_subcuboid.x1..(cur_off_subcuboid.x2+1) {
                for y_coord in cur_off_subcuboid.y1..(cur_off_subcuboid.y2+1) {
                    for z_coord in cur_off_subcuboid.z1..(cur_off_subcuboid.z2+1) {

                        let mut found_intersect: bool = false;

                        if sub_cuboid_idx > 0 {
                            // check cur coord against all prev cuboid ranges
                            for prev_off_subcuboid in off_subcuboids[0..sub_cuboid_idx].iter() {
                                // if not intersect
                                if
                                (x_coord >= prev_off_subcuboid.x1 && x_coord <= prev_off_subcuboid.x2) &&
                                (y_coord >= prev_off_subcuboid.y1 && y_coord <= prev_off_subcuboid.y2) &&
                                (z_coord >= prev_off_subcuboid.z1 && z_coord <= prev_off_subcuboid.z2)
                                {
                                    found_intersect = true;
                                    break;
                                }
                            }
                        }

                        // check against all on_subcuboids as well
                        if !found_intersect {
                            for on_subcuboid in on_subcuboids.iter() {
                                // if not intersect
                                if
                                (x_coord >= on_subcuboid.x1 && x_coord <= on_subcuboid.x2) &&
                                (y_coord >= on_subcuboid.y1 && y_coord <= on_subcuboid.y2) &&
                                (z_coord >= on_subcuboid.z1 && z_coord <= on_subcuboid.z2)
                                {
                                    found_intersect = true;
                                    break;
                                }
                            }   
                        }

                        if !found_intersect {
                            off_cube_count += 1;
                        }
                    }
                }
            }
        }

        // vol += cube_vol - ignore_cube_set.len() - ignore_cube_count - off_cube_set.len() - off_cube_count
        num_on_cubes += cuboid_vol(cur_cuboid) - ignore_cube_count - off_cube_count;
    }

    // println!("on_set.len(): {}", on_set.len());
    println!("result: {}", num_on_cubes);

}