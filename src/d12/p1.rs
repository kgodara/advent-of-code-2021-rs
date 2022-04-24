use crate::util::file;

use std::collections::{ HashMap, HashSet };

use std::{ rc::Rc, cell::RefCell };

use std::fmt;

const STEPS: u64 = 100;

#[derive(Clone, Debug, PartialEq)]
enum NodeType {
    Start,
    End,
    BigCave,
    SmallCave,
}

#[derive(Clone, Debug)]
struct Cave {
    pub name: Rc<String>,
    pub n_type: NodeType,
    pub adj_caves: Vec<Rc<RefCell<Cave>>>,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {:?}, adj_caves: {} ]", self.name, self.n_type, self.adj_caves.len())
    }
}

pub fn exec() {
    let src: String = file::read_file_arg();

    let mut cave_lookup: HashMap<&str, Rc<RefCell<Cave>>> = HashMap::new();

    cave_lookup.insert("start", Rc::new(RefCell::new(Cave { name: Rc::new(String::from("start")), n_type: NodeType::Start, adj_caves: vec![] })));
    cave_lookup.insert("end", Rc::new(RefCell::new(Cave { name: Rc::new(String::from("end")), n_type: NodeType::End, adj_caves: vec![] })));

    // populate Octopus grid
    for line in src.lines() {

        let mut cave_edge = line.split('-');

        let cave_one_label: &str = cave_edge.next().unwrap();
        let cave_two_label: &str = cave_edge.next().unwrap();

        let is_cave_one_big: bool = cave_one_label.to_ascii_uppercase() == cave_one_label;
        let is_cave_two_big: bool = cave_two_label.to_ascii_uppercase() == cave_two_label;

        // get_or_insert will never occur for "start" and "end" caves since pre-inserted
        // cave_lookup.get(k: &Q)
        let cave_one = cave_lookup.get(cave_one_label);
        if cave_one.is_none() {
            cave_lookup.insert(cave_one_label,
                Rc::new(
                    RefCell::new(
                        Cave { name: Rc::new(cave_one_label.to_string()), n_type: if is_cave_one_big { NodeType::BigCave } else { NodeType::SmallCave }, adj_caves: vec![]}
                    )
                )
            );
        }

        let cave_two = cave_lookup.get(cave_two_label);
        if cave_two.is_none() {
            cave_lookup.insert(cave_two_label,
                Rc::new(
                    RefCell::new(
                        Cave { name: Rc::new(cave_two_label.to_string()), n_type: if is_cave_two_big { NodeType::BigCave } else { NodeType::SmallCave }, adj_caves: vec![]}
                    )
                )
            );
        }

        let cave_one = cave_lookup.get(cave_one_label).unwrap();
        let cave_two = cave_lookup.get(cave_two_label).unwrap();

        cave_one.borrow_mut().adj_caves.push(Rc::clone(cave_two));
        cave_two.borrow_mut().adj_caves.push(Rc::clone(cave_one));

    }



    let mut visited_small_cave_stack: Vec<(Rc<RefCell<Cave>>, HashSet<Rc<String>>)> = Vec::new();

    let start_cave = cave_lookup.get_mut("start").unwrap();

    // push all caves accessible from the start to the stack
    for cave in start_cave.borrow().adj_caves.iter() {
        visited_small_cave_stack.push((Rc::clone(cave), HashSet::from_iter(vec![Rc::clone(&cave.borrow().name)].iter().cloned())));
    }


    let mut unique_path_num: u64 = 0;

    // Assumption: a big cave will never be connected to another big cave (no adjacent big caves)
    while !visited_small_cave_stack.is_empty() {
        let cur_cave_tuple = visited_small_cave_stack.pop().unwrap();
        let cur_cave = cur_cave_tuple.0.borrow();
        let mut cur_cave_visited = cur_cave_tuple.1;

        for adj_cave in cur_cave.adj_caves.iter() {
            let adj_cave_b = adj_cave.borrow();
            // is adj_cave an unvisited small cave or a big cave?
            if (adj_cave_b.n_type == NodeType::SmallCave && !cur_cave_visited.contains(&adj_cave_b.name)) ||
                adj_cave_b.n_type == NodeType::BigCave {

                // println!("cur_cave {} visiting adj_cave {}", cur_cave, adj_cave_b);

                if cur_cave.n_type == NodeType::SmallCave {
                    cur_cave_visited.insert(Rc::clone(&cur_cave.name));
                }

                visited_small_cave_stack.push(( Rc::clone(adj_cave), cur_cave_visited.clone() ));
            }

            // is adj_cave the end
            else if adj_cave_b.n_type == NodeType::End {
                // println!("cur_cave {} visiting end", cur_cave);
                unique_path_num += 1;
            }
        }
        
    }
    println!("result: {}", unique_path_num);

}