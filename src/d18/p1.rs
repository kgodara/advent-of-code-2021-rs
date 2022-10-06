#[derive(Clone, Copy, Debug)]
enum TreeElem {
    Value(u8),
    Parent,
}

// Accepts: idx of element to search relative to
// Returns: idx
fn find_next_val(num_tree: &Vec<Option<TreeElem>>, ref_idx: u8, search_first_left: bool, next_val_stack: &mut [u8; 32]) -> Option<u8> {

    next_val_stack[0] = 0;
    let mut stack_size: u8 = 1;


    let mut cur_node_idx: u8;
    let mut ref_node_seen: bool = false;

    let mut last_seen_val_idx: Option<u8> = None;

    // Look for leftmost num >= 10
    while stack_size != 0 {
        cur_node_idx = next_val_stack[(stack_size-1) as usize];
        stack_size -= 1;

        if cur_node_idx == ref_idx && search_first_left {
            return last_seen_val_idx;
        }

        match num_tree[cur_node_idx as usize].unwrap() {
            TreeElem::Parent => {
                // push left child after, traverse left -> right
                next_val_stack[stack_size as usize] = cur_node_idx * 2 + 2;
                stack_size += 1;

                next_val_stack[stack_size as usize] = cur_node_idx * 2 + 1;
                stack_size += 1;
            },
            TreeElem::Value(_val) => {
                if ref_node_seen && !search_first_left {
                    return Some(cur_node_idx)
                }
                last_seen_val_idx = Some(cur_node_idx);
            },
        }

        ref_node_seen = cur_node_idx == ref_idx || ref_node_seen;
        
    }
    None
}

fn exec_explode(num_tree: &mut Vec<Option<TreeElem>>, dfs_stack: &mut [u8; 32]) -> bool {

    // Search for first eligible pair to explode:
    // Depths and Indexes:
    //     Depth-1, Index, 0-0
    //     Depth-2, Index, 1-2
    //     Depth-3, Index, 3-6
    //     Depth-4, Index, 7-14
    //     Depth-5, Index, 15-30
    // The first TreeElem::Parent in ranges 15-31 is target to explode

    /*
    [ 1-2
        [ 3-6
            [ 7-14
                [ 15-31
                    [9,8],
                    1
                ],
                2
            ],
            3
        ],
        4
    ]
    */



    // Get idx of first explodable element
    let mut to_explode_idx: u8 = 0;
    let mut explode_left: u8 = 0;
    let mut explode_right: u8 = 0;

    for (idx, elem_opt) in num_tree.iter().enumerate().skip(15) {
        if idx > 31 { break; }

        if let Some(TreeElem::Parent) = elem_opt {

            // Get Child values & Replace Pair with 0
            explode_left = match num_tree[idx * 2 + 1].unwrap() {
                TreeElem::Value(val) => {val},
                TreeElem::Parent => {panic!("Two levels of invalid pairs found!")}
            };

            explode_right = match num_tree[idx * 2 + 2].unwrap() {
                TreeElem::Value(val) => {val},
                TreeElem::Parent => {panic!("Two levels of invalid pairs found!")}
            };


            elem_opt.unwrap();

            to_explode_idx = idx as u8;
            break;
        }
    }

    if to_explode_idx == 0 { return false; }

    // Replace Parent & children with 0

    num_tree[to_explode_idx as usize] = Some(TreeElem::Value(0));
    num_tree[(to_explode_idx * 2 + 1) as usize] = None;
    num_tree[(to_explode_idx * 2 + 2) as usize] = None;
    

    // find idx of leftmost element, do addition if found
    if let Some(first_left_elem) = find_next_val(&num_tree, to_explode_idx, true, dfs_stack) {

        num_tree[first_left_elem as usize] = match num_tree[first_left_elem as usize].unwrap() {
            TreeElem::Parent => {panic!("Can't increment value")},
            TreeElem::Value(val) => {
                Some(TreeElem::Value(explode_left+val))
            }
        };
    }


    // find idx of rightmost element, do addition if found
    if let Some(first_right_elem) = find_next_val(&num_tree, to_explode_idx, false, dfs_stack) {

        num_tree[first_right_elem as usize] = match num_tree[first_right_elem as usize].unwrap() {
            TreeElem::Parent => {panic!("Can't increment value")},
            TreeElem::Value(val) => {
                Some(TreeElem::Value(explode_right+val))}
        };
    }

    
    true
}

fn exec_split(num_tree: &mut Vec<Option<TreeElem>>, dfs_stack: &mut [u8; 32]) -> bool {


    // Traverse the tree left -> right, use a stack (need LIFO)
    dfs_stack[0] = 0;
    let mut stack_size: u8 = 1;

    let mut cur_node_idx: u8;

    // Look for leftmost num >= 10
    while stack_size != 0 {
        cur_node_idx = dfs_stack[(stack_size-1) as usize];
        stack_size -= 1;

        match num_tree[cur_node_idx as usize].unwrap() {
            TreeElem::Parent => {
                // push left child after, traverse left -> right
                dfs_stack[stack_size as usize] = cur_node_idx * 2 + 2;
                stack_size += 1;

                dfs_stack[stack_size as usize] = cur_node_idx * 2 + 1;
                stack_size += 1;
            },
            TreeElem::Value(val) => {
                if val >= 10 {

                    /*
                        To split a regular number, replace it with a pair;
                        the left element of the pair should be the regular number divided by two
                        and rounded down, while the right element of the pair should be the
                        regular number divided by two and rounded up.
                    */
                    num_tree[cur_node_idx as usize] = Some(TreeElem::Parent);
                    num_tree[(cur_node_idx * 2 + 1) as usize] = Some(TreeElem::Value(val/2));
                    num_tree[(cur_node_idx * 2 + 2) as usize] = Some(TreeElem::Value((val/2) + (val%2)));

                    return true;
                }
            },
        }
    }

    false
}

fn get_magnitude(num_tree: &Vec<Option<TreeElem>>, start_idx: u8) -> u64 {

    match num_tree[start_idx as usize].unwrap() {
        TreeElem::Parent => {
            (3 * get_magnitude(num_tree, start_idx*2+1)) +
            (2 * get_magnitude(num_tree, start_idx*2+2))
        },
        TreeElem::Value(val) => {val as u64},
    }
}


fn _print_tree_vals(tree: &Vec<Option<TreeElem>>) {
    println!("( ");

    let mut dfs_stack: Vec<usize> = vec![0];

    let mut cur_node_idx: usize;
    while !dfs_stack.is_empty() {
        cur_node_idx = dfs_stack.pop().unwrap();

        match tree[cur_node_idx].unwrap() {
            TreeElem::Parent => {
                // push left child after, traverse left -> right
                dfs_stack.push(cur_node_idx * 2 + 2);
                dfs_stack.push(cur_node_idx * 2 + 1);
            },
            TreeElem::Value(val) => {
                if cur_node_idx >= 1 && cur_node_idx <= 2 {
                    print!("  ");
                } else if cur_node_idx >= 3 && cur_node_idx <= 6 {
                    print!("    ");
                } else if cur_node_idx >= 7 && cur_node_idx <= 14 {
                    print!("      ");
                } else if cur_node_idx >= 15 && cur_node_idx <= 30 {
                    print!("        ");
                } else if cur_node_idx >= 31 && cur_node_idx <= 62 {
                    print!("          ");
                }
                println!("{}", val);
            }
        }
    }
    println!(")");
}

pub fn exec(src: &str, print: bool) {

    let mut input_nums: Vec<Vec<Option<TreeElem>>> = vec![];

    // No single root node
    let mut cur_idx = 0;


    let mut cur_num: Vec<Option<TreeElem>>;

    for line in src.lines() {
        // for easy indexing, constant is just an estimate
        // max depth is 4, thus including leaf & internal nodes: (2^4+2^3+2^2+2^1+2^0) --> 31 max nodes
        cur_num = vec![None; 31];

        for ch in line.chars() {

            if ch.is_digit(10) {
                cur_num[cur_idx] = Some(TreeElem::Value(ch.to_digit(10).unwrap() as u8));
            }
            else {
                match ch {
                    '[' => {
                        // descending a level down tree: *2 + 1
                        cur_num[cur_idx] = Some(TreeElem::Parent);
                        cur_idx = cur_idx * 2 + 1
                    },
                    ',' => {
                        // sibling: + 1
                        cur_idx += 1;
                    },
                    ']' => {
                        // move back up to parent: (...-1) / 2
                        cur_idx = (cur_idx-1) / 2;
                    },
                    _ => { panic!("invalid char: [{}]", ch) }
                }
            }
        }

        input_nums.push(cur_num);
    }

    // will always have reduced num
    let mut result: Vec<Option<TreeElem>>;

    // can have up to depth 5, while numbers are being exploded/split
    let mut to_merge: Vec<Option<TreeElem>> = vec![None; 63];

    // Compute results for each line
    // merge result with next snail num

    let mut generic_stack: [u8; 32] = [0; 32];

    result = input_nums[0].clone();

    for snail_num in input_nums.iter().skip(1) {
        // merge binary trees [result, snail_num]

        // populate to_merge:
        //     result: left-half of each level 
        //     snail_num: right-half of each level
        //     everything is moved one level down due to merging

        to_merge[0] = Some(TreeElem::Parent);

        // root node (depth=0) already handled
        for depth in 1..6 {
            // let depth_len: usize = 2usize.pow(depth);

            // fill each level
            // Note: both of these are inclusive
            // merge depth 2 (3-6) pulls from src depth 1 [1-2, 1-2]
            let src_depth_start = 2usize.pow(depth-1)-1;
            let src_depth_end = src_depth_start * 2;

            let merge_depth_half_len = 2usize.pow(depth)/2;
            let merge_depth_first_half = 2usize.pow(depth)-1;
            let merge_depth_second_half = merge_depth_first_half + merge_depth_half_len;

            to_merge[merge_depth_first_half..merge_depth_first_half+merge_depth_half_len].copy_from_slice(
                &result[src_depth_start..(src_depth_end+1)]
            );

            to_merge[merge_depth_second_half..merge_depth_second_half+merge_depth_half_len].copy_from_slice(
                &snail_num[src_depth_start..(src_depth_end+1)]
            );
        }


        let mut changed: bool = true;

        while changed {
            changed = exec_explode(&mut to_merge, &mut generic_stack);
            if !changed {
                changed = exec_split(&mut to_merge, &mut generic_stack);
            }
        }

        // move now valid to_merge data into 'result' & reset to_merge
        result[0..31].copy_from_slice(&to_merge[0..31]);
        for val in to_merge.iter_mut() {
            *val = None;
        }
    }

    if print { println!("result: {}", get_magnitude(&result, 0)) }
}