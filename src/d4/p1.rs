use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct BoardCell {
    pub row_idx: usize,
    pub col_idx: usize,
    pub board_idx: usize,
}

fn calc_score(board_idx: usize, final_num: u32, sol_board: &[Vec<Vec<bool>>], boards: &[Vec<Vec<u32>>]) -> u64 {
    // iterate over sol_board:
    //     fetch marked status using board_idx
    //     if unmarked, fetch value from boards, add to sum
    // multiply sum by final num & print

    let mut sum: u64 = 0;

    for (row_idx, row) in sol_board.iter().enumerate() {
        for (col_idx, _col) in row.iter().enumerate() {
            if !sol_board[row_idx][col_idx][board_idx] {
                sum += boards[row_idx][col_idx][board_idx] as u64;
            }
        }
    }

    sum*(final_num as u64)

}

fn parse_board_and_nums(src: String,
    nums_to_draw: &mut Vec<u32>,
    board_num: &mut usize,
    boards: &mut Vec<Vec<Vec<u32>>>,
    cell_val_lookup: &mut HashMap<u32, Vec<BoardCell>>
) {


    let mut board_idx: u32 = 0;
    let mut board_line_num: Option<u32> = None;

    let mut row_nums: Vec<u32>;

    for (idx, line) in src.lines().enumerate() {
        // nums_to_draw - 1,45,23,4,...
        if idx == 0 {
            nums_to_draw.append(
                &mut (
                    line.split(',')
                        .map(|num_str| { num_str.parse::<u32>().unwrap() })
                        .collect()
                )
            );

        } else if idx == 1 {
            // skip first blank line
            continue;
        } else {
            // transition between boards
            if line.trim().is_empty() {
                board_line_num = None;
                board_idx += 1;
            } else {
                // increment board row idx
                if board_line_num.is_none() {
                    board_line_num = Some(0);
                } else {
                    board_line_num = Some(board_line_num.unwrap()+1);
                }
                
                // board row
                row_nums = line.trim()
                    .split_ascii_whitespace()
                    .map(|num_str| { num_str.parse().unwrap() })
                    .collect();

                for (col_idx, num) in row_nums.iter().enumerate() {

                    let new_val_index_board = BoardCell {
                        row_idx: board_line_num.unwrap() as usize,
                        col_idx,
                        board_idx: board_idx as usize
                    };

                    // update cell_val_lookup
                    if cell_val_lookup.get(num).is_none() {
                        cell_val_lookup.insert(*num, vec![ new_val_index_board ]);
                    } else if let Some(num_vec) = cell_val_lookup.get_mut(num) {
                        num_vec.push(new_val_index_board);
                    }
                    boards[board_line_num.unwrap() as usize][col_idx].push(*num);
                }
            }
        }
    }
    *board_num = (board_idx+1) as usize;
}

fn check_valid_bingo(sol_board: &[Vec<Vec<bool>>],
    check_rows: bool,
    seqs_to_check: &HashSet<usize>,
    boards_to_check: &[(usize, bool)],
    winner_board_idx: &mut usize) -> bool {
    // horizontal|vertical
    // check for row|col bingos
    // check only rows in (rows|cols)_to_check
    // check only boards in boards_to_check

    let mut remaining_seq_boards: Vec<(usize, bool)>;

    let mut cell: &Vec<bool>;

    // check only rows|cols in (rows|cols)_to_check
    for seq_idx in seqs_to_check.iter() {

        remaining_seq_boards = Vec::from_iter(boards_to_check.iter().cloned());

        // iter over individual row/col cells

        for cell_idx in if check_rows { 0..sol_board[*seq_idx].len() } else { 0..sol_board.len() } {

            cell = if check_rows { &sol_board[*seq_idx][cell_idx] } else { &sol_board[cell_idx][*seq_idx] };

            let mut board_idx: &usize;

            remaining_seq_boards = remaining_seq_boards.into_iter().filter(|board_tup| { board_tup.1 }).collect();

            // check only boards in boards_to_check
            for board_idx_idx in 0..remaining_seq_boards.len() {
                board_idx = &remaining_seq_boards.get(board_idx_idx).unwrap().0;

                // cell unmarked, board no longer relevant for this row|col
                // update remaining_(row|col)_boards
                if !cell[*board_idx] {
                    remaining_seq_boards[board_idx_idx] = (*board_idx, false);
                }
            }
        }
        remaining_seq_boards = remaining_seq_boards.into_iter().filter(|board_tup| { board_tup.1 }).collect();
        if !remaining_seq_boards.is_empty() {
            *winner_board_idx = remaining_seq_boards.get(0).unwrap().0;
            return true;
        }
    }
    false
}



pub fn exec(src: String) {
    // Represent all boards in one super-board
    //     which tracks the boards marked at each cell
    // On number draw:
    //     Iterate over 3-dimensional vector (super-board of given boards) to fetch boards to be marked
    //     Mark boards and begin search

    let mut nums_to_draw: Vec<u32> = Vec::new();
    let mut board_num: usize = 0;

    // vec of rows -> vec of numbers
    let mut boards: Vec<Vec<Vec<u32>>> = vec![vec![vec![]; 5]; 5];

    let mut cell_val_lookup: HashMap<u32, Vec<BoardCell>> = HashMap::new();


    // (nums_to_draw, num_boards, boards)
    // (Vec<u32>, u32, Vec<Vec<Vec<u32>>>, HashMap<u32, Vec<BoardCell>>)
    parse_board_and_nums(src, &mut nums_to_draw, &mut board_num, &mut boards, &mut cell_val_lookup);

    // 5x5 bingo boards
    // vec<row> [ vec<col> [ vec<board_idx>[ is_marked ] ] ]
    let mut sol_board: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; board_num]; 5]; 5];

    // vec of board_idx's to check for bingo
    let mut boards_to_check: Vec<(usize, bool)> = Vec::new();

    let mut rows_to_check: HashSet<usize> = HashSet::new();
    let mut cols_to_check: HashSet<usize> = HashSet::new();

    let mut last_drawn_num: u32 = 0;

    let mut winner_board_idx: usize = 0;

    // Draw numbers
    for drawn_num in nums_to_draw.iter() {
        last_drawn_num = *drawn_num;
        // Find boards to be marked
        if let Some(board_hits) = cell_val_lookup.get(drawn_num) {

            // mark relevant boards and
            // identify list of rows and columns to check for matches from mark_details
            for mark_detail in board_hits.iter() {
                sol_board[mark_detail.row_idx][mark_detail.col_idx][mark_detail.board_idx] = true;

                boards_to_check.push((mark_detail.board_idx, true));

                rows_to_check.insert(mark_detail.row_idx);
                cols_to_check.insert(mark_detail.col_idx);
            }

            // Search rows/cols
            if check_valid_bingo(&sol_board, true, &rows_to_check, &boards_to_check, &mut winner_board_idx) { break }
            if check_valid_bingo(&sol_board, false, &cols_to_check, &boards_to_check, &mut winner_board_idx) { break }

        }

        boards_to_check.drain(..);
    }

    println!("result: {}", calc_score(winner_board_idx,
        last_drawn_num,
        &sol_board,
        &boards
    ));
}