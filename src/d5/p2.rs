use std::cmp::max;

#[derive(Debug, PartialEq)]
enum LineType {
    Row,
    Col,
    Diag
}

#[derive(Debug)]
struct Line {

    // is this a (row|col) line or a diagonal
    pub line_type: LineType,

    pub x_bound_one: usize,
    pub y_bound_one: usize,

    pub x_bound_two: usize,
    pub y_bound_two: usize,
}


pub fn exec(src: String) {

    let mut lines: Vec<Line> = Vec::new();

    let mut line_type: LineType;

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

        max_x = max(max_x, max(start_coords[0], end_coords[0]));
        max_y = max(max_y, max(start_coords[1], end_coords[1]));
    

        line_type = if start_coords[1] == end_coords[1] {
            LineType::Row
        } else if start_coords[0] == end_coords[0] {
            LineType::Col
        } else {
            LineType::Diag
        };

        lines.push(Line { line_type,
            x_bound_one: start_coords[0],
            y_bound_one: start_coords[1],

            x_bound_two: end_coords[0],
            y_bound_two: end_coords[1],
        });
    }

    // +1 since upper_bounds are inclusive
    let mut field: Vec<Vec<usize>> = vec![vec![0; max_x+1]; max_y+1];

    let mut cell_x_idx: usize;
    let mut cell_y_idx: usize = 0;

    let mut final_x;
    let mut final_y;

    for line in lines.iter() {

        let is_x_bound_incr: bool = line.x_bound_one < line.x_bound_two;
        let is_y_bound_incr: bool = line.y_bound_one < line.y_bound_two;

        cell_x_idx = 0;

        if line.line_type == LineType::Diag {
            cell_y_idx = 0;
        }

        // upper bound is inclusive
        // Problem: The direction matters for diagonal lines,
        // meaning that the bounds have to be iterated over in from start -> end coords,
        // direction can't be changed 

        // Ex: (8, 4) -> (5, 1)
        // Iter over abs( difference )
        //     decide whether to incr or decr relative to (x|y)_bound_one based on if starting at larger|smaller
        // 
        while (cell_x_idx as i32) <= ((line.x_bound_one as i32) - (line.x_bound_two as i32)).abs() {


            // if non-diagonal, need to set cell_y_idx here in case of horizontal line
            // since inner loop will only be executing once per outer loop iter
            if line.line_type != LineType::Diag {
                cell_y_idx = 0;
            }

            while (cell_y_idx as i32) <= ((line.y_bound_one as i32) - (line.y_bound_two as i32)).abs() {


                if is_x_bound_incr {
                    final_x = line.x_bound_one + cell_x_idx;
                } else {
                    final_x = line.x_bound_one - cell_x_idx;
                }

                if is_y_bound_incr {
                    final_y = line.y_bound_one + cell_y_idx;
                } else {
                    final_y = line.y_bound_one - cell_y_idx;
                }

                field[final_y][final_x] += 1;

                cell_y_idx += 1;
                if line.line_type == LineType::Diag {
                    break;
                }
            }
            cell_x_idx += 1;
        }
    }

    let mut total: usize = 0;
    for row in field.iter() {
        for cell in row.iter() {
            if *cell > 1 { total += 1 }
        }
    }

    println!("result: {:?}", total);

}