// TODO: Improvement input parsing

use std::cmp::{min, max};

use std::collections::{ HashMap };

#[derive(Hash, Clone, PartialEq, Eq)]
struct Cuboid {
    x1: i32,
    x2: i32,

    y1: i32,
    y2: i32,

    z1: i32,
    z2: i32,
}

fn print_cuboid(c: &Cuboid) {
    println!("({}, {}), ({}, {}), ({}, {})", c.x1, c.x2, c.y1, c.y2, c.z1, c.z2);
}

fn has_intersect(c_one: &Cuboid, c_two: &Cuboid) -> bool {
    (c_two.x1 <= c_one.x2 && c_two.x2 >= c_one.x1) &&
    (c_two.y1 <= c_one.y2 && c_two.y2 >= c_one.y1) &&
    (c_two.z1 <= c_one.z2 && c_two.z2 >= c_one.z1)
}

// Note: add max() so double negative ranges don't produce > 0 vol
fn cuboid_vol(cuboid: &Cuboid) -> i64 {
    (max(cuboid.x2 - cuboid.x1 + 1, 0) as i64) *
    (max(cuboid.y2 - cuboid.y1 + 1, 0) as i64) *
    (max(cuboid.z2 - cuboid.z1 + 1, 0) as i64)
}

pub fn exec(src: String) {
    let lines = src.lines();

    let mut cur_state: HashMap<Cuboid, i32> = HashMap::new();

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

        let cur_cuboid: Cuboid = Cuboid {
            x1: x_range[0], x2: x_range[1],
            y1: y_range[0], y2: y_range[1],
            z1: z_range[0], z2: z_range[1],
        };

        // check for intersections

        // Note: avoiding another HashMap to represent updated state gave a 20x speedup
        // (key cloning was required to gen hashset of cuboids to unify current and updated states)
        let mut update_tuples: Vec<(Cuboid, i32)> = vec![];

        for (cuboid, val) in cur_state.iter() {

            if has_intersect(cuboid, &cur_cuboid) {
                let intersect_cuboid = Cuboid {
                    x1: max(cur_cuboid.x1, cuboid.x1), x2: min(cur_cuboid.x2, cuboid.x2),
                    y1: max(cur_cuboid.y1, cuboid.y1), y2: min(cur_cuboid.y2, cuboid.y2),
                    z1: max(cur_cuboid.z1, cuboid.z1), z2: min(cur_cuboid.z2, cuboid.z2),
                };

                update_tuples.push((intersect_cuboid, *val));
            }
        }

        for tup in update_tuples.into_iter() {
            *cur_state.entry(tup.0).or_insert(0) -= tup.1;
        }

        if is_on_cmd {
            *cur_state.entry(cur_cuboid).or_insert(0) += 1;
        }
    }

    let mut result: i64 = 0;

    for (cuboid, val) in cur_state.iter() {
        result += cuboid_vol(cuboid) * (*val as i64);
    }

    println!("result: {}", result);

}