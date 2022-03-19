use crate::util::file;
use std::cmp::max;

#[derive(Debug)]
struct Line {
    pub is_row: bool,

    // idx for range which doesn't change
    pub primary_idx: usize,

    pub bound_one: usize,
    pub bound_two: usize,
}




pub fn exec() {
    let src: String = file::read_file_arg();

    let mut lines: Vec<Line> = Vec::new();
    let mut is_row: bool;

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    
    for line in src.lines() {
       let pairs: Vec<&str> = line.split(" -> ").collect();

       let start_coords: Vec<usize> = pairs[0].split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect();

        let end_coords: Vec<usize> = pairs[1].split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect();

        is_row = if start_coords[1] == end_coords[1] {
            true
        } else if start_coords[0] == end_coords[0] {
            false
        } else {
            continue;
        };

        max_x = max(max_x, max(start_coords[0], end_coords[0]));
        max_y = max(max_y, max(start_coords[1], end_coords[1]));
        
        lines.push(Line { is_row,
            primary_idx: if is_row { start_coords[1] } else { start_coords[0] },
            bound_one: if is_row { start_coords[0] } else { start_coords[1] },
            bound_two: if is_row { end_coords[0] } else { end_coords[1] }
        });
    }


    // +1 since upper_bounds are inclusive
    let mut field: Vec<Vec<usize>> = vec![vec![0; max_x+1]; max_y+1];

    for line in lines.iter() {
        // upper bound is inclusive
        for cell_idx in 
        if line.bound_one < line.bound_two { line.bound_one..(line.bound_two+1) } else { line.bound_two..(line.bound_one+1) } {

            if line.is_row {
                field[line.primary_idx][cell_idx] += 1;
            } else {
                field[cell_idx][line.primary_idx] += 1;
            }
        }
    }

    let mut total: usize = 0;
    for row in field.iter() {
        for cell in row.iter() {
            if *cell > 1 { total += 1 }
        }
    }

    println!("Total: {:?}", total);


}