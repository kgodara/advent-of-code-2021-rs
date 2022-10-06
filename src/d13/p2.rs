use std::rc::Rc;
use std::cell::RefCell;

use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

pub fn exec(src: &str, print: bool) {

    let mut sorted_x: Vec<Rc<RefCell<Point>>> = Vec::new();
    let mut sorted_y: Vec<Rc<RefCell<Point>>> = Vec::new();

    let mut is_sorted: bool = false;

    // parse caves
    for line in src.lines() {

        if line.contains(',') {
            let mut l_split = line.split(',');

            let new_point: Rc<RefCell<Point>> = Rc::new( RefCell::new( Point { x: l_split.next().unwrap().parse().unwrap(), y: l_split.next().unwrap().parse().unwrap() } ) );

            sorted_x.push(Rc::clone(&new_point));
            sorted_y.push(Rc::clone(&new_point));
        }

        if line.contains("fold along") {

            if !is_sorted {
                sorted_x.sort_unstable_by(|a, b| a.borrow().x.cmp(&b.borrow().x));
                sorted_y.sort_unstable_by(|a, b| a.borrow().y.cmp(&b.borrow().y));
                is_sorted = true;
            }

            let is_x_fold: bool = line.contains("x=");
            let fold_val: u16 = line.split('=').last().unwrap().parse().unwrap();

            // Assumption: There are never any dots on the axes being folded
            if is_x_fold {
                for dot in sorted_x.iter_mut() {
                    let mut dot_mut = dot.borrow_mut();
                    if dot_mut.x > fold_val {
                        dot_mut.x = fold_val - (dot_mut.x - fold_val);
                    }
                }
                sorted_x.sort_unstable_by(|a, b| a.borrow().x.cmp(&b.borrow().x));
            } else {
                for dot in sorted_y.iter_mut() {
                    let mut dot_mut = dot.borrow_mut();
                    if dot_mut.y > fold_val {
                        dot_mut.y = fold_val - (dot_mut.y - fold_val);
                    }
                }
                sorted_y.sort_unstable_by(|a, b| a.borrow().y.cmp(&b.borrow().y));
            }

            sorted_x.sort_unstable_by(|a, b| {
                    match a.borrow().x.cmp(&b.borrow().x) {
                        Ordering::Less => { Ordering::Less },
                        Ordering::Greater => { Ordering::Greater },
                        Ordering::Equal => { a.borrow().y.cmp(&b.borrow().y) }
                    }
                }
            );
        }
    }


    let mut max_x: u16 = 0;
    let mut max_y: u16 = 0;

    for dot in sorted_x.iter() {
        if max_x < dot.borrow().x { max_x = dot.borrow().x }
        if max_y < dot.borrow().y { max_y = dot.borrow().y }
    }

    let mut out_grid = vec![vec!['.'; (max_x+1) as usize]; (max_y+1) as usize];

    for dot in sorted_x.iter() {
        out_grid[dot.borrow().y as usize][dot.borrow().x as usize] = '*';
    }
    if print {
        println!("result: ");
        for row in out_grid.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }

}