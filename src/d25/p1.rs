#![allow(clippy::collapsible_if)]

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    East,
    South,
    Empty
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::East => { write!(f, ">") },
            Cell::South=> { write!(f, "v") },
            Cell::Empty => { write!(f, ".") },
        }
    }
}

fn _print_grid(grid: &[Vec<Cell>]) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

pub fn exec(src: &str, print: bool) {

    // let mut grid: Vec<Vec<Cell>> = vec![];
    let mut grid: [[Cell; 139]; 137] = [[Cell::Empty; 139]; 137];

    for (line_idx, line) in src.lines().enumerate() {
        // let mut cur_row: Vec<Cell> = vec![];

        for (ch_idx, ch) in line.bytes().enumerate() {
            grid[line_idx][ch_idx] = match ch {
                b'v' => {Cell::South},
                b'>' => {Cell::East},
                b'.' => {Cell::Empty},
                _ => {panic!("invalid cell val")}
            };
        }
        // grid.push(cur_row);
    }

    /*
    let mut grid_next: Vec<Vec<Cell>> = vec![];
    for row in grid_cur.iter() {
        grid_next.push(vec![Cell::Empty; row.len()]);
    }
    */

    // println!("INIT:");
    // print_grid(&grid);

    // keep iterating until no changes
    let mut step_count: u64 = 0;
    loop {
        let mut moved: bool = false;

        // Process east movement (cols)
        for (_row_idx, row) in grid.iter_mut().enumerate() {
            let left_bound_state: Cell = row[0].clone();

            let mut cell_idx: usize = 0;

            // Note: if move east happens, incr cell_idx to prevent double processing
            while cell_idx < (row.len()-1) {
                if row[cell_idx] == Cell::East && row[cell_idx+1] == Cell::Empty {

                    // println!("EAST: grid[{}][{}] --> grid[{}][{}] ", row_idx, cell_idx, row_idx, cell_idx+1);

                    moved = true;

                    row[cell_idx] = Cell::Empty;
                    row[cell_idx+1] = Cell::East;
                    cell_idx += 1;
                }
                cell_idx += 1;
            }

            // handle right-most cell, if nothing moved into it (it wasn't empty)
            if cell_idx < row.len() {

                let row_len = row.len();

                if row[row.len()-1] == Cell::East && left_bound_state == Cell::Empty {

                    // println!("EAST: grid[{}][{}] --> grid[{}][{}] ", row_idx, row_len-1, row_idx, 0);

                    moved = true;

                    row[row_len-1] = Cell::Empty;
                    row[0] = Cell::East;
                }
            }
        }



        // Process south movement (rows)
        // Assumption: uniform-length rows
        for col_idx in 0..(grid[0].len()) {

            let top_bound_state: Cell = grid[0][col_idx].clone();

            let mut cell_idx: usize = 0;

            // Note: if move south happens, incr cell_idx to prevent double processing
            while cell_idx < (grid.len()-1) {
                if grid[cell_idx][col_idx] == Cell::South && grid[cell_idx+1][col_idx] == Cell::Empty {
                    moved = true;

                    grid[cell_idx][col_idx] = Cell::Empty;
                    grid[cell_idx+1][col_idx] = Cell::South;
                    cell_idx += 1;
                }
                cell_idx += 1;
            }

            // handle bottom-most cell, if nothing moved into it (it wasn't empty)
            if cell_idx < grid.len() {

                let grid_len = grid.len();

                if grid[grid.len()-1][col_idx] == Cell::South && top_bound_state == Cell::Empty {
                    moved = true;
                    grid[grid_len-1][col_idx] = Cell::Empty;
                    grid[0][col_idx] = Cell::South;
                }
            }
        }

        if !moved { break; }
        step_count += 1;

        // println!("AFTER STEP: {}", step_count);
        // print_grid(&grid);
    }

    if print { println!("result: {:?}", step_count+1) }   

}