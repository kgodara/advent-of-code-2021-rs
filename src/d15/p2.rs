use crate::util::file;

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use std::cmp::Ordering;

use std::collections::{ HashSet, BinaryHeap };

#[derive(Clone, Debug)]
pub struct Node<T> {

    pub label: u64,
    pub val: T,

    // dist from source
    pub dist: u32,

    pub adj_list: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> fmt::Display for Node<T>
where 
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} adj_list: {} ]", self.val, self.adj_list.len())
    }
}

impl<T> Ord for Node<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl<T> PartialOrd for Node<T> where T: Ord + Eq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<T> Eq for Node<T> {}

pub fn exec() {
    let src: String = file::read_file_arg();
    let mut grid: Vec<Vec<Rc<RefCell<Node<u8>>>>> = Vec::new();

    let mut min_heap: BinaryHeap<Rc<RefCell<Node<u8>>>> = BinaryHeap::new();
    let mut node_idx: u64 = 0;

    for line in src.lines() {

        grid.push(line.chars().map(|ch| {
            let new_node = Rc::new( RefCell::new( Node {
                label: node_idx,
                val: ch.to_digit(10).unwrap() as u8,
                dist: u32::MAX,
                adj_list: vec![] }
            ));
            node_idx += 1;

            new_node
        }).collect());
    }


    // tile grid to 5x5 square:
    #[allow(clippy::type_complexity)]
    let mut tiled_grid: Vec<Vec<Option<Rc<RefCell<Node<u8>>>>>> = vec![vec![None; grid[0].len()*5]; grid.len()*5];

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[0].len() {

            for cell_row_idx in 0..5 {
                for cell_col_idx in 0..5 {

                    let mut new_val: u8 = grid[row_idx][col_idx].borrow().val + (cell_row_idx as u8) + (cell_col_idx as u8);

                    // wrap around to 1
                    if new_val > 9 {
                        new_val -= 9;
                    }

                    let new_node = Rc::new(
                        RefCell::new(
                            Node {
                                label: node_idx,
                                val: new_val,
                                dist: u32::MAX,
                                adj_list: vec![]
                            }
                        )
                    );
                    node_idx += 1;
                    tiled_grid[row_idx+(cell_row_idx*grid.len())][col_idx+(cell_col_idx*grid[0].len())] = Some(new_node);

                }
            }
        }
    }

    let grid: Vec<Vec<Rc<RefCell<Node<u8>>>>> = tiled_grid.into_iter()
        .map(|row_vec| {
            row_vec.into_iter()
            .map(|cell_opt| cell_opt.unwrap())
            .collect()
        })
        .collect();

    // populate adjacency lists
    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[0].len() {

            // left cell
            if row_idx > 0 {
                grid[row_idx][col_idx].borrow_mut().adj_list.push(Rc::clone(&grid[row_idx-1][col_idx]));
            }

            // right cell
            if row_idx < (grid.len()-1) {
                grid[row_idx][col_idx].borrow_mut().adj_list.push(Rc::clone(&grid[row_idx+1][col_idx]));
            }

            // top cell
            if col_idx > 0 {
                grid[row_idx][col_idx].borrow_mut().adj_list.push(Rc::clone(&grid[row_idx][col_idx-1]));
            }

            // bottom cell
            if col_idx < (grid[0].len()-1) {
                grid[row_idx][col_idx].borrow_mut().adj_list.push(Rc::clone(&grid[row_idx][col_idx+1]));
            }
        }
    }

    // set start_node dist = 0
    grid[0][0].borrow_mut().dist = 0;

    let mut visited_nodes: HashSet<u64> = HashSet::new();

    // Djikstra's, could also use A* with Manhattan distance
    // Note: instead of reallocating the min_heap on every iter
    //     don't populate the min_heap with all nodes initially
    //     and add nodes as their distances are improved and
    //     filter out any already visited nodes
    min_heap.push(Rc::clone(&grid[0][0]));

    let mut cur_node: Rc<RefCell<Node<u8>>>;

    while !min_heap.is_empty() {

        cur_node = min_heap.pop().unwrap();

        if !visited_nodes.insert(cur_node.borrow().label) {
            // already visited this node
            continue;
        }


        if Rc::ptr_eq(&cur_node, &grid[grid.len()-1][grid[0].len()-1]) {
            break;
        }

        for adj_node in cur_node.borrow().adj_list.iter() {
            let new_dist: u32 = cur_node.borrow().dist + (adj_node.borrow().val as u32);
            if new_dist < adj_node.borrow().dist {
                adj_node.borrow_mut().dist = new_dist;

                min_heap.push(Rc::clone(adj_node));
            }
        }
    }

    println!("result: {}", grid[grid.len()-1][grid[0].len()-1].borrow().dist);

    
}