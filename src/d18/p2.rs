// TODO: Explore using one loop for all explode() val searches, e.g.
//          combining with loop for finding 'to_explode_idx'

// TODO: Change explode to 'explode_all()', explicitly track all split() indexes


#[derive(Clone, Copy, Debug)]
enum TreeElem {
    Value(u8),
    Parent,
}

// Accepts: idx of element to search relative to
// Returns: idx
fn find_adj_vals(num_tree: &Vec<Option<TreeElem>>, ref_idx: u8, next_val_stack: &mut [u8; 32])
-> (Option<u8>, Option<u8>) {

    next_val_stack[0] = 0;
    let mut stack_size: u8 = 1;


    let mut cur_node_idx: u8;
    let mut ref_node_seen: bool = false;

    // (left_adj, right_adj)
    let mut result: (Option<u8>, Option<u8>) = (None, None);

    // Look for leftmost num >= 10
    while stack_size != 0 {
        cur_node_idx = next_val_stack[(stack_size-1) as usize];
        stack_size -= 1;

        match num_tree[cur_node_idx as usize].unwrap() {
            TreeElem::Parent => {
                // push left child after, traverse left -> right
                next_val_stack[stack_size as usize] = cur_node_idx * 2 + 2;
                stack_size += 1;

                next_val_stack[stack_size as usize] = cur_node_idx * 2 + 1;
                stack_size += 1;
            },
            TreeElem::Value(_val) => {
                if ref_node_seen {
                    result.1 = Some(cur_node_idx);
                    return result
                } else if cur_node_idx != ref_idx {
                    result.0 = Some(cur_node_idx);
                }
            },
        }

        ref_node_seen = cur_node_idx == ref_idx || ref_node_seen;
        
    }
    result
}

fn explode(num_tree: &mut Vec<Option<TreeElem>>, dfs_stack: &mut [u8; 32]) -> bool {

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

    let adj_vals = find_adj_vals(&num_tree, to_explode_idx, dfs_stack);


    // find idx of leftmost element, do addition if found
    if let Some(first_left_elem) = adj_vals.0 {// find_next_val(&num_tree, to_explode_idx, true, dfs_stack) {

        num_tree[first_left_elem as usize] = match num_tree[first_left_elem as usize].unwrap() {
            TreeElem::Parent => {panic!("Can't increment value")},
            TreeElem::Value(val) => {
                Some(TreeElem::Value(explode_left+val))
            }
        };
    }


    // find idx of rightmost element, do addition if found
    if let Some(first_right_elem) = adj_vals.1 {// find_next_val(&num_tree, to_explode_idx, false, dfs_stack) {

        num_tree[first_right_elem as usize] = match num_tree[first_right_elem as usize].unwrap() {
            TreeElem::Parent => {panic!("Can't increment value")},
            TreeElem::Value(val) => {
                Some(TreeElem::Value(explode_right+val))}
        };
    }

    
    true
}

fn split(num_tree: &mut Vec<Option<TreeElem>>, dfs_stack: &mut [u8; 32]) -> bool {


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

                    // split() only called when no explode actions possible,
                    // so if new parent is too deep, can call explode with known idx
                    // else can continue splitting

                    // only stop splitting if need to explode
                    if cur_node_idx >= 15 {
                        return true;
                    }
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

    let mut input_nums: [[Option<TreeElem>; 31]; 100] = [[None; 31]; 100];
    let mut cur_num_idx: u8 = 0;

    // No single root node
    let mut cur_idx = 0;

    for line in src.lines() {
        // for easy indexing, constant is just an estimate
        // max depth is 4, thus including leaf & internal nodes: (2^4+2^3+2^2+2^1+2^0) --> 31 max nodes
        // cur_num = vec![None; 31];

        for ch in line.bytes() {

            // utf-8 digit offset
            if ch >= 48 && ch < 58 {
                input_nums[cur_num_idx as usize][cur_idx] = Some(TreeElem::Value(ch - 48));
            }
            else {
                match ch {
                    b'[' => {
                        // descending a level down tree: *2 + 1
                        input_nums[cur_num_idx as usize][cur_idx] = Some(TreeElem::Parent);
                        cur_idx = cur_idx * 2 + 1
                    },
                    b',' => {
                        // sibling: + 1
                        cur_idx += 1;
                    },
                    b']' => {
                        // move back up to parent: (...-1) / 2
                        cur_idx = (cur_idx-1) / 2;
                    },
                    _ => { panic!("invalid char: [{}]", ch) }
                }
            }
        }

        cur_num_idx += 1;
    }

    // can have up to depth 5, while numbers are being exploded/split
    let mut to_merge: Vec<Option<TreeElem>> = vec![None; 63];

    // Compute results for each line
    // merge result with next snail num

    // let mut generic_stack: Vec<usize> = vec![0; 20];

    // even in their invalid merged state, snail numbers can only provide a max of 32 values
    let mut generic_stack: [u8; 32] = [0; 32];

    let mut max_mag: u64 = 0;

    for (left_idx, left_num) in input_nums.iter().enumerate() {
        for (right_idx, right_num) in input_nums.iter().enumerate() {

            if left_idx == right_idx { continue; }

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
                    &left_num[src_depth_start..(src_depth_end+1)]
                );

                to_merge[merge_depth_second_half..merge_depth_second_half+merge_depth_half_len].copy_from_slice(
                    &right_num[src_depth_start..(src_depth_end+1)]
                );
            }


            let mut changed: bool = true;

            while changed {
                changed = explode(&mut to_merge, &mut generic_stack);
                if !changed {
                    changed = split(&mut to_merge, &mut generic_stack);
                }
            }

            max_mag = std::cmp::max(max_mag, get_magnitude(&to_merge, 0));

        }
    }

    if print { println!("result: {}", max_mag) }
}