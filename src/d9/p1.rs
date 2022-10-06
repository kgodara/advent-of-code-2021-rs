pub fn exec(src: &str, print: bool) {

    let mut height_grid: Vec<Vec<u64>> = vec![];

    for line in src.lines() {
        let mut height_row: Vec<u64> = Vec::new();
        for height in line.chars() {
            height_row.push(height.to_string().parse().unwrap());
        }
        height_grid.push(height_row);
    }

    let mut risk_level_sum: u64 = 0;

    let mut top_cell: Option<u64>;
    let mut bottom_cell: Option<u64>;

    let mut left_cell: Option<u64>;
    let mut right_cell: Option<u64>;

    let mut is_invalid_cell: bool = false;

    for (row_idx, row) in height_grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {

            left_cell = if col_idx > 0 {
                Some(height_grid[row_idx][col_idx-1])
            } else {
                None
            };

            // Assumption: square height grid
            right_cell = if col_idx < (height_grid[0].len() - 1) {
                Some(height_grid[row_idx][col_idx+1])
            } else {
                None
            };

            top_cell = if row_idx > 0 {
                Some(height_grid[row_idx-1][col_idx])
            } else {
                None
            };

            bottom_cell = if row_idx < (height_grid.len() - 1) {
                Some(height_grid[row_idx+1][col_idx])
            } else {
                None
            };



            if let Some(left_val) = left_cell {
                if left_val <= *cell {
                    is_invalid_cell = true;
                }
            }

            if let Some(right_val) = right_cell {
                if right_val <= *cell {
                    is_invalid_cell = true;
                }
            }

            if let Some(top_val) = top_cell {
                if top_val <= *cell {
                    is_invalid_cell = true;
                }
            }

            if let Some(bottom_val) = bottom_cell {
                if bottom_val <= *cell {
                    is_invalid_cell = true;
                }
            }

            if !is_invalid_cell {
                // println!("Cell ({}, {}) --> {} is low point", row_idx, col_idx, cell);
                risk_level_sum += cell + 1;
            }

            is_invalid_cell = false;

        }
    }

    if print { println!("result: {:?}", risk_level_sum) }



}