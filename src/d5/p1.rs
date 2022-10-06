use std::cmp::max;

#[derive(Debug)]
struct Line {
    pub is_row: bool,

    // idx for range which doesn't change
    pub primary_idx: u16,

    pub bound_one: u16,
    pub bound_two: u16,

    // pub bound_range: Range<u16>,
}

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;



pub fn exec(src: &str, print: bool) {

    let mut lines: Vec<Line> = Vec::new();
    let mut is_row: bool;

    let mut max_x: u16 = 0;
    let mut max_y: u16 = 0;
    
    for line in src.lines() {
        
        let all_coords: Vec<u16> = line.split(" -> ")
            .map(|coord_pair| {
                coord_pair.split(',')
            })
            .flatten()
            .map(|coord_str| {
                coord_str.parse().unwrap()
            })
            .collect();

        // start_coords[1] == end_coords[1]
        is_row = if all_coords[1] == all_coords[3] {
            true
        }
        // start_coords[0] == end_coords[0]
        else if all_coords[0] == all_coords[2] {
            false
        } else {
            continue;
        };

        max_x = max(max_x, max(all_coords[0], all_coords[2]));
        max_y = max(max_y, max(all_coords[1], all_coords[3]));

        lines.push(Line { is_row,
            primary_idx: if is_row { all_coords[1] } else { all_coords[0] },
            bound_one: if is_row { all_coords[0] } else { all_coords[1] },
            bound_two: if is_row { all_coords[2] } else { all_coords[3] }
        });
    }

    // +1 since upper_bounds are inclusive
    let mut field: [[u16; MAX_X]; MAX_Y] = [[0; MAX_X]; MAX_Y];

    for line in lines.iter() {
        // upper bound is inclusive
        for cell_idx in 
        if line.bound_one < line.bound_two { line.bound_one..(line.bound_two+1) } else { line.bound_two..(line.bound_one+1) } {

            if line.is_row {
                field[line.primary_idx as usize][cell_idx as usize] += 1;
            } else {
                field[cell_idx as usize][line.primary_idx as usize] += 1;
            }
        }
    }

    let mut total: usize = 0;
    for row in field.iter() {
        for cell in row.iter() {
            if *cell > 1 { total += 1 }
        }
    }

    if print { println!("result: {:?}", total) }
}