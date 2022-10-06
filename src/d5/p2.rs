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

    pub x_bound_one: i32,
    pub y_bound_one: i32,

    pub x_bound_two: i32,
    pub y_bound_two: i32,
}

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;


pub fn exec(src: &str, print: bool) {

    let mut lines: Vec<Line> = Vec::new();

    let mut line_type: LineType;

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
        line_type = if all_coords[1] == all_coords[3] {
            LineType::Row
        }
        // start_coords[0] == end_coords[0]
        else if all_coords[0] == all_coords[2] {
            LineType::Col
        } else {
            LineType::Diag
        };

        max_x = max(max_x, max(all_coords[0], all_coords[2]));
        max_y = max(max_y, max(all_coords[1], all_coords[3]));
    

        lines.push(Line { line_type,
            x_bound_one: all_coords[0] as i32,
            y_bound_one: all_coords[1] as i32,

            x_bound_two: all_coords[2] as i32,
            y_bound_two: all_coords[3] as i32,
        });
    }

    // +1 since upper_bounds are inclusive
    let mut field: [[u16; MAX_X]; MAX_Y] = [[0; MAX_X]; MAX_Y];

    let mut cell_x_idx: i32;
    let mut cell_y_idx: i32 = 0;

    let mut final_x;
    let mut final_y;

    for line in lines.iter() {

        let x_bound_incr: i32 = if line.x_bound_one < line.x_bound_two { 1 } else { -1 };
        let y_bound_incr: i32 = if line.y_bound_one < line.y_bound_two { 1 } else { -1 };

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


        final_x = line.x_bound_one;
        final_y = line.y_bound_one;

        while cell_x_idx <= (line.x_bound_one - line.x_bound_two).abs() {


            // if non-diagonal, need to set cell_y_idx here in case of horizontal line
            // since inner loop will only be executing once per outer loop iter
            if line.line_type != LineType::Diag {
                cell_y_idx = 0;
                final_y = line.y_bound_one;
            }

            while cell_y_idx <= (line.y_bound_one - line.y_bound_two).abs() {

                field[final_y as usize][final_x as usize] += 1;

                cell_y_idx += 1;
                final_y += y_bound_incr;

                if line.line_type == LineType::Diag {
                    break;
                }
            }
            cell_x_idx += 1;
            final_x += x_bound_incr;
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