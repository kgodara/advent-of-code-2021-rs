fn find_new_basin_idx(height_grid: &[Vec<(u64, bool)>]) -> Option<(u64, u64)> {

    for (row_idx, height_row) in height_grid.iter().enumerate() {
        let new_basin_col_opt: Option<usize> = height_row.iter()
            .position(|cell_tuple| { !cell_tuple.1 });

        if let Some(new_basin_col_idx) = new_basin_col_opt {
            return Some((row_idx as u64, new_basin_col_idx as u64));
        }
    }
    None
}

pub fn exec(src: String) {

    // (val, basin_checked)
    let mut height_grid: Vec<Vec<(u64, bool)>> = vec![];

    for line in src.lines() {
        let mut height_row: Vec<(u64, bool)> = Vec::new();
        for height in line.chars() {
            let parsed_val: u64 = height.to_string().parse().unwrap();

            // 9 is never part of a basin
            height_row.push((parsed_val, parsed_val == 9));
        }
        height_grid.push(height_row);
    }

    // (row_idx, col_idx)
    let mut cell_stack: Vec<(u64, u64)> = Vec::new();

    let first_non_nine_idx: (u64, u64) = find_new_basin_idx(&height_grid).unwrap();

    height_grid[(first_non_nine_idx.0 as usize)][first_non_nine_idx.1 as usize].1 = true;
    cell_stack.push(first_non_nine_idx);

    let mut cur_cell: (u64, u64);
    let mut cur_basin_size: u64 = 0;
    let mut basin_size_list: Vec<u64> = Vec::new();

    while !cell_stack.is_empty() {
        cur_cell = cell_stack.pop().unwrap();
        cur_basin_size += 1;
        
        // cur_cell.1 -> col_idx
        // left cell
        if cur_cell.1 > 0 {
            // if left cell has not been basin checked
            if !height_grid[(cur_cell.0 as usize)][(cur_cell.1-1) as usize].1 {
                height_grid[(cur_cell.0 as usize)][(cur_cell.1-1) as usize].1 = true;
                cell_stack.push((cur_cell.0, cur_cell.1-1));
            }
        }

        // cur_cell.1 -> col_idx
        // right cell
        if cur_cell.1 < (height_grid[0].len()-1) as u64 {
            // if right cell has not been basin checked
            if !height_grid[(cur_cell.0 as usize)][(cur_cell.1+1) as usize].1 {
                height_grid[(cur_cell.0 as usize)][(cur_cell.1+1) as usize].1 = true;
                cell_stack.push((cur_cell.0, cur_cell.1+1));

            }
        }

        // cur_cell.0 -> row_idx
        // top cell
        if cur_cell.0 > 0 {
            // if top cell has not been basin checked
            if !height_grid[((cur_cell.0-1) as usize)][cur_cell.1 as usize].1 {
                height_grid[((cur_cell.0-1) as usize)][cur_cell.1 as usize].1 = true;
                cell_stack.push((cur_cell.0-1, cur_cell.1));
            }
        }

        // cur_cell.0 -> row_idx
        // bottom cell
        if cur_cell.0  < (height_grid.len() - 1) as u64 {
            // if bottom cell has not been basin checked
            if !height_grid[((cur_cell.0+1) as usize)][cur_cell.1 as usize].1 {
                height_grid[((cur_cell.0+1) as usize)][cur_cell.1 as usize].1 = true;
                cell_stack.push((cur_cell.0+1, cur_cell.1));
            }
        }

        if cell_stack.is_empty() {
            basin_size_list.push(cur_basin_size);
            cur_basin_size = 0;
            if let Some(cell_idx_tuple) = find_new_basin_idx(&height_grid) {

                height_grid[(cell_idx_tuple.0 as usize)][cell_idx_tuple.1 as usize].1 = true;
                cell_stack.push(cell_idx_tuple);
            }
        }


    }

    basin_size_list.sort_unstable();
    basin_size_list.reverse();

    println!("result: {}", basin_size_list[0] * basin_size_list[1] * basin_size_list[2]);
}