use std::collections::{ HashMap, hash_map::Entry };

use std::{ rc::Rc, cell::RefCell };

use std::fmt;

// Rust String Interner: https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html
#[derive(Default)]
pub struct Interner {
    pub map: HashMap<String, u8>,
    pub vec: Vec<String>,
}

impl Interner {
    pub fn intern(&mut self, name: &str) -> u8 {

        let idx = self.map.len() as u8;
        match self.map.entry(name.to_owned()) {
            Entry::Occupied(entry) => { *entry.get() },
            Entry::Vacant(entry) => {
                entry.insert(idx);
                // Don't care about reconstructing strings
                // self.vec.push(name.to_owned());
                idx
            }
        }
    }

    pub fn lookup(&self, idx: u8) -> &str {
        self.vec[idx as usize].as_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum NodeType {
    Start,
    End,
    BigCave,
    SmallCave,
}

#[derive(Clone, Debug)]
struct Cave {
    pub name: u8,
    pub n_type: NodeType,
    pub adj_caves: Vec<Rc<RefCell<Cave>>>,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {:?}, adj_caves: {} ]", self.name, self.n_type, self.adj_caves.len())
    }
}

fn mark_visited(visited_log: &mut u16, cave: &u8) {
    *visited_log |= 1 << cave;
}

fn is_visited(visited_log: &u16, cave: &u8) -> bool {
    (visited_log >> cave) & 1 == 1
}

// TODO: Explore perf improvement

pub fn exec(src: &str, print: bool) {

    let mut cave_label_interner: Interner = Interner::default();



    let mut cave_lookup: HashMap<u8, Rc<RefCell<Cave>>> = HashMap::new();

    // intern "start" and "end" at 0 & 1
    cave_label_interner.intern("start");
    cave_label_interner.intern("end");

    cave_lookup.insert(0, Rc::new(RefCell::new(Cave { name: 0, n_type: NodeType::Start, adj_caves: vec![] })));
    cave_lookup.insert(1, Rc::new(RefCell::new(Cave { name: 1, n_type: NodeType::End, adj_caves: vec![] })));

    // parse caves
    for line in src.lines() {

        let mut cave_edge = line.split('-');

        let cave_one_label: &str = cave_edge.next().unwrap();
        let cave_two_label: &str = cave_edge.next().unwrap();

        let cave_one_label_interned = cave_label_interner.intern(cave_one_label);
        let cave_two_label_interned = cave_label_interner.intern(cave_two_label);

        let is_cave_one_big: bool = cave_one_label.to_ascii_uppercase() == cave_one_label;
        let is_cave_two_big: bool = cave_two_label.to_ascii_uppercase() == cave_two_label;

        // get_or_insert will never occur for "start" and "end" caves since pre-inserted
        let cave_one = match cave_lookup.entry(cave_one_label_interned) {
            Entry::Occupied(entry) => { Rc::clone(entry.get()) },
            Entry::Vacant(entry) => {
                Rc::clone(
                    entry.insert(Rc::new(
                        RefCell::new(
                            Cave { name: cave_one_label_interned, n_type: if is_cave_one_big { NodeType::BigCave } else { NodeType::SmallCave }, adj_caves: vec![]}
                        )
                    ))
                )
            }
        };

        let cave_two = match cave_lookup.entry(cave_two_label_interned) {
            Entry::Occupied(entry) => { Rc::clone(entry.get()) },
            Entry::Vacant(entry) => {
                Rc::clone(
                    entry.insert(Rc::new(
                        RefCell::new(
                            Cave { name: cave_two_label_interned, n_type: if is_cave_two_big { NodeType::BigCave } else { NodeType::SmallCave }, adj_caves: vec![]}
                        )
                    ))
                )
            }
        };

        cave_one.borrow_mut().adj_caves.push(Rc::clone(&cave_two));
        cave_two.borrow_mut().adj_caves.push(cave_one);

    }

    // (cur_cave, visited_small_caves, have_visited_twice)
    let mut visited_small_cave_stack: Vec<(Rc<RefCell<Cave>>, u16, bool)> = Vec::new();

    let start_cave = cave_lookup.get_mut(&0).unwrap();

    visited_small_cave_stack.push((Rc::clone(start_cave), 0, false));

    let mut unique_path_num: u64 = 0;

    // Assumption: a big cave will never be connected to another big cave (no adjacent big caves)
    while !visited_small_cave_stack.is_empty() {
        let cur_cave_tuple = visited_small_cave_stack.pop().unwrap();

        let cur_cave = cur_cave_tuple.0.borrow();
        let mut cur_cave_visited = cur_cave_tuple.1;
        let mut have_visited_twice: bool = cur_cave_tuple.2;

        if cur_cave.n_type == NodeType::SmallCave {
            // if this is the first visit to cur_cave
            if !is_visited(&cur_cave_visited, &cur_cave.name) {
                mark_visited(&mut cur_cave_visited, &cur_cave.name);
            }
            // second visit to cur_cave, pass have_visited_twice as true
            else {
                have_visited_twice = true;
            }
        }

        for adj_cave in cur_cave.adj_caves.iter() {
            let adj_cave_b = adj_cave.borrow();
            let adj_cave_b_visited = is_visited(&cur_cave_visited, &adj_cave_b.name);

            // is adj_cave:
            //     the start node
            //     an unvisited small cave
            //     a visited small cave, but haven't visited a small cave twice yet
            //     a big cave
            if  (adj_cave_b.n_type == NodeType::SmallCave && (!adj_cave_b_visited || !have_visited_twice) ) ||
                adj_cave_b.n_type == NodeType::BigCave {
                visited_small_cave_stack.push(( Rc::clone(adj_cave), cur_cave_visited, have_visited_twice ));
            }

            // is adj_cave the end
            else if adj_cave_b.n_type == NodeType::End {
                unique_path_num += 1;
            }
        }
    }

    if print { println!("result: {}", unique_path_num) }

}