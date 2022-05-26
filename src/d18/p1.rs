use std::fmt;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Element {
    // can't have a literal as a parent
    parent: Option<Rc<RefCell<Element>>>,
    value: Option<Rc<RefCell<u8>>>,
    children: Vec<Rc<RefCell<Element>>>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(literal_val) = &self.value {
            write!(f, "{}", literal_val.borrow())?;
        } else {
            if self.children.len() != 2 {
                panic!("Invalid Pair, must have at two children");
            }
            write!(f, "[")?;

            write!(f, "{}", self.children[0].borrow())?;
            
            write!(f, ", ")?;

            write!(f, "{}", self.children[1].borrow())?;

            write!(f, "]")?;
        }
        Ok(())
    }
}

// (element, depth)
type ElementWithDepth = (Rc<RefCell<Element>>, u8);

fn exec_explode(dfs_stack: &mut Vec<ElementWithDepth>) -> bool {
    // Literal
    let mut left_explode_target_opt: Option<Rc<RefCell<Element>>> = None;

    let mut cur_elem: ElementWithDepth;
    let mut cur_depth: u8;

    let mut searching_right_literal: bool = false;
    let mut right_literal_to_add: u8 = 0;

    while !dfs_stack.is_empty() {
        cur_elem = dfs_stack.pop().unwrap();
        cur_depth = cur_elem.1;

        if searching_right_literal {

            if let Some(literal_val_ref) = &cur_elem.0.borrow().value {
                *literal_val_ref.borrow_mut() += right_literal_to_add;
                return true;
            } else {
                // push children in reversed order
                dfs_stack.push((Rc::clone(&cur_elem.0.borrow().children[1]), cur_depth+1));
                dfs_stack.push((Rc::clone(&cur_elem.0.borrow().children[0]), cur_depth+1));
            }

            continue;
        }

        let left_explode_val: u8;

        // Determine operations to perform and set params
        // Pair
        if cur_elem.0.borrow().value.is_none() {

            // When evaluating pair where left child is >= 10:
            //     do explode since otherwise would have to try to add a pair
            if cur_depth >= 4 {

                ( left_explode_val, right_literal_to_add ) = ( *cur_elem.0.borrow().children[0].borrow().value.as_ref().unwrap().borrow(), *cur_elem.0.borrow().children[1].borrow().value.as_ref().unwrap().borrow());

                if let Some(left_explode_target) = &left_explode_target_opt {
                    if let Some(literal_ref) = &left_explode_target.borrow_mut().value {
                        *literal_ref.borrow_mut() += left_explode_val;
                    }
                }

                let cur_parent = cur_elem.0.borrow().parent.clone();

                *cur_elem.0.borrow_mut() = Element {
                    parent: cur_parent,
                    value: Some(Rc::new(RefCell::new(0))),
                    children: vec![],
                };
                searching_right_literal = true;
            } else {
                dfs_stack.push((Rc::clone(&cur_elem.0.borrow().children[1]), cur_depth+1));
                dfs_stack.push((Rc::clone(&cur_elem.0.borrow().children[0]), cur_depth+1));
            }


        } else {
            left_explode_target_opt = Some(Rc::clone(&cur_elem.0));
        }
    }
    false
}

fn exec_split(dfs_stack: &mut Vec<ElementWithDepth>) -> bool {


    let mut cur_elem: ElementWithDepth;
    let mut cur_depth: u8;

    while !dfs_stack.is_empty() {
        cur_elem = dfs_stack.pop().unwrap();
        cur_depth = cur_elem.1;

        if cur_elem.0.borrow().value.is_some() {

            let literal_val_ref = &cur_elem.0.borrow().value.clone().unwrap();
            if *literal_val_ref.borrow() > 9 {

                let val_to_split: u8 = *literal_val_ref.borrow();

                let left_literal: u8 = val_to_split / 2;
                let right_literal: u8 = (val_to_split / 2) + (val_to_split % 2);

                let cur_parent = cur_elem.0.borrow().parent.clone();

                *cur_elem.0.borrow_mut() = Element {
                    parent: cur_parent,
                    value: None,
                    children: vec![
                        Rc::new(RefCell::new(Element { parent: Some(Rc::clone(&cur_elem.0)), value: Some(Rc::new(RefCell::new(left_literal))), children: vec![] })),
                        Rc::new(RefCell::new(Element { parent: Some(Rc::clone(&cur_elem.0)), value: Some(Rc::new(RefCell::new(right_literal))), children: vec![] })),                                
                    ]
                };

                // No need to push to stack, change made, can break
                return true;
            }
        } else {
            dfs_stack.push((Rc::clone(&cur_elem.0.borrow().children[1]), cur_depth+1));
            dfs_stack.push((Rc::clone(&cur_elem.0.borrow().children[0]), cur_depth+1));
        }
    }
    false
}

fn get_magnitude(elem: Rc<RefCell<Element>>) -> u64 {
    if elem.borrow().value.is_some() {
        *elem.borrow().value.as_ref().unwrap().borrow() as u64
    } else {
        (3*get_magnitude(Rc::clone(&elem.borrow().children[0]))) + (2*get_magnitude(Rc::clone(&elem.borrow().children[1])))
    }
}

pub fn exec(src: String) {

    let mut input_data: Vec<Rc<RefCell<Element>>> = vec![];

    // parse input numbers
    for line in src.lines() {

        let mut cur_pair: Rc<RefCell<Element>> = Rc::new(RefCell::new(Element { parent: None, value: None, children: vec![] }));

        // skip the first opening bracket, already assumed
        for ch in line.chars().skip(1) {
            if ch.is_digit(10) {
                cur_pair.borrow_mut().children.push(Rc::new(RefCell::new(
                    Element {
                        parent: Some(Rc::clone(&cur_pair)),
                        value: Some(Rc::new(RefCell::new(ch.to_digit(10).unwrap() as u8))),
                        children: vec![],
                    }
                )));
            }
            else {
                match ch {
                    '[' => {

                        let new_child_pair = Rc::new(RefCell::new(Element {
                            parent: Some(Rc::clone(&cur_pair)),
                            value: None,
                            children: vec![],
                        }));

                        cur_pair.borrow_mut().children.push(Rc::clone(&new_child_pair));
                        cur_pair = Rc::clone(&new_child_pair);
                    },
                    ',' => {
                    },
                    // move back up to parent
                    ']' => {
                        #[allow(clippy::needless_late_init)]
                        let temp;
                        // assignment on matched variable causes borrow-checker confusion
                        match &cur_pair.borrow().parent {
                            Some(parent_element) => { temp = Rc::clone(parent_element); },
                            None => { break; },
                        }

                        cur_pair = temp;
                    },
                    _ => { panic!("invalid char: [{}]", ch) }
                }
            }
        }

        input_data.push(cur_pair);
    }



    // Compute results for each line
    // merge result with next snail num
    let mut result: Rc<RefCell<Element>> = Rc::clone(&input_data[0]);

    for snail_num in input_data.iter().skip(1) {

        result = Rc::new(RefCell::new(Element {
            parent: None,
            value: None,
            children: vec![ Rc::clone(&result), Rc::clone(snail_num) ]
        }));

        let mut changed: bool = true;
        // While iteration yields changes:
        //     DFS iterate through number
        while changed {

            // Note: result will always have 2 children, both of which must be pairs
            // push in reversed order in order to traverse left --> right
            let mut dfs_stack: Vec<ElementWithDepth> = vec![(Rc::clone(&result.borrow().children[1]), 1), (Rc::clone(&result.borrow().children[0]), 1)];

            // set result's children's parents
            let result_parent = Rc::clone(&result);
            result.borrow_mut().children[0].borrow_mut().parent = Some(result_parent.clone());
            result.borrow_mut().children[1].borrow_mut().parent = Some(result_parent);

            changed = exec_explode(&mut dfs_stack);
            if !changed {
                dfs_stack = vec![(Rc::clone(&result.borrow().children[1]), 1), (Rc::clone(&result.borrow().children[0]), 1)];
                changed = exec_split(&mut dfs_stack);
            }
        }
    }

    println!("result: {}", get_magnitude(Rc::clone(&result)));

}