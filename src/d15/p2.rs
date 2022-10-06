use std::fmt;

use std::collections::{ BinaryHeap };
use std::cmp::Ordering;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Node<T> {

    pub idx: u32,
    pub val: T,

    // dist from source
    pub dist: u32,
}

impl<T> fmt::Display for Node<T>
where 
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} idx: ({}) ]", self.val, self.idx)
    }
}
// (dist, idx)
pub struct MinHeapTuple(u32, usize);

impl Ord for MinHeapTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for MinHeapTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl PartialEq for MinHeapTuple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MinHeapTuple {}

pub fn exec(src: &str, print: bool) {
    let mut orig_grid: Vec<Vec<Node<u8>>> = Vec::new();

    let mut node_idx: u32 = 0;

    for line in src.lines() {

        orig_grid.push(
            line.bytes()
                .map(|ch| {
                    let new_node = Node {
                        idx: 0,
                        // utf-8 digit offset
                        val: ch-48,
                        // val: ch.to_digit(10).unwrap() as u8,
                        dist: u32::MAX
                    };
                    node_idx += 1;
                    new_node
                })
                .collect()
        );
    }

    let orig_row_len = orig_grid.len();
    let orig_col_len = orig_grid[0].len();

    // tile grid to 5x5 square:

    let mut grid: Vec<Node<u8>> = Vec::with_capacity(250_000);

    let new_row_size = orig_col_len * 5;

    for i in 0..(new_row_size * orig_row_len*5) {
        // map i: tiled_grid_idx -> relevant element from orig grid

        // tiled_grid_row_idx % orig_row_len
        let orig_row_idx = (i / new_row_size) % orig_row_len;

        // col_idx just increments in a mod system
        let orig_col_idx = i % orig_col_len;

        let new_row_idx = i / new_row_size;
        let new_col_idx = i % new_row_size;
    
        let mut new_val: u8 = orig_grid[orig_row_idx][orig_col_idx].val + 
            ((new_row_idx / orig_row_len) as u8) + 
            ((new_col_idx / orig_col_len) as u8);

        if new_val > 9 {
            new_val -= 9;
        }

        let new_node = 
            Node {
                idx: i as u32,
                val: new_val,
                dist: u32::MAX,
            };

        node_idx += 1;

        grid.push(new_node);
    }

    let pop_adj_arr = |
        node: &Node<u8>,
        orig_row_len: &usize,
        orig_col_len: &usize,
        adj_arr: &mut [usize; 4],
        adj_arr_size: &mut usize| 
    {

        let new_row_size = orig_col_len * 5;
        let new_row_num = orig_row_len * 5;

        let cur_idx = node.idx as usize;

        // top cell - not in top row:
        if cur_idx >= new_row_size {
            adj_arr[*adj_arr_size] = cur_idx-new_row_size;
            *adj_arr_size += 1;
        }

        // bottom cell - not in bottom row:
        if cur_idx < ((new_row_num * new_row_size) - new_row_size) {
            adj_arr[*adj_arr_size] = cur_idx+new_row_size;
            *adj_arr_size += 1;
        }

        // left cell - not on left edge
        if (cur_idx % new_row_size) != 0 {
            adj_arr[*adj_arr_size] = cur_idx-1;
            *adj_arr_size += 1;
        }

        // right cell - not on right edge
        if ((cur_idx+1) % new_row_size) != 0 {
            adj_arr[*adj_arr_size] = cur_idx+1;
            *adj_arr_size += 1;
        }
    };

    // set start_node dist = 0
    grid[0].dist = 0;

    let mut visited_nodes: Vec<bool> = vec![false; node_idx as usize];

    // (dist, idx)
    let mut min_heap: BinaryHeap<MinHeapTuple> = BinaryHeap::new();

    // Djikstra's, could also use A* with Manhattan distance
    // Note: instead of reallocating the min_heap on every iter
    //     don't populate the min_heap with all nodes initially
    //     and add nodes as their distances are improved and
    //     filter out any already visited nodes
    min_heap.push(MinHeapTuple(grid[0].dist, 0));

    let mut cur_tuple: MinHeapTuple;
    let mut cur_node: &Node<u8>;

    // store indexes of adjacent nodes
    let mut cur_adj: [usize; 4] = Default::default();
    let mut adj_arr_size: usize;

    let mut adj_node: &mut Node<u8>;

    while !min_heap.is_empty() {

        cur_tuple = min_heap.pop().unwrap();
        cur_node = &grid[cur_tuple.1];

        let idx = cur_node.idx as usize;


        let cur_node_dist = cur_node.dist;


        if visited_nodes[idx] {
            // already visited this node
            continue;
        } else {
            visited_nodes[idx] = true;
        }

        if *cur_node == grid[grid.len()-1] {
            break;
        }

        adj_arr_size = 0;
        pop_adj_arr(cur_node, &orig_row_len, &orig_col_len, &mut cur_adj, &mut adj_arr_size);

        for adj_node_idx in cur_adj[0..adj_arr_size].iter() {
            adj_node = &mut grid[*adj_node_idx];

            let new_dist: u32 = cur_node_dist + (adj_node.val as u32);
            if new_dist < adj_node.dist {
                adj_node.dist = new_dist;

                min_heap.push(MinHeapTuple(adj_node.dist, *adj_node_idx));
            }
        }
    }

    if print { println!("result: {}", grid[grid.len()-1].dist) }


}