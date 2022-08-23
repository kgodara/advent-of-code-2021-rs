use std::{ rc::Rc, cell::RefCell };

const STEPS: u64 = 2000;

fn reset_flashed_bool(octopi_grid: &mut Vec<Vec<Rc<RefCell<Octopus>>>>) {

    for row in octopi_grid {
        for cell in row {
            cell.borrow_mut().flashed = false;
        }
    }
}

fn incr_all(octopi_grid: &mut Vec<Vec<Rc<RefCell<Octopus>>>>) {

    for row in octopi_grid {
        for cell in row {
            cell.borrow_mut().energy += 1;
        }
    }
}

fn get_all_to_flash(octopi_grid: &mut Vec<Vec<Rc<RefCell<Octopus>>>>) -> Vec<Rc<RefCell<Octopus>>> {
    let mut result: Vec<Rc<RefCell<Octopus>>> = Vec::new();
    for row in octopi_grid {
        for cell in row {
            if cell.borrow().energy > 9 {
                result.push(Rc::clone(cell));
            }
        }
    }
    result
}



#[derive(Clone, Debug)]
struct Octopus {
    pub energy: u64,

    pub flashed: bool,

    pub adj_octopi: Vec<Rc<RefCell<Octopus>>>,
}

pub fn exec(src: String) {

    let mut octopi_grid: Vec<Vec<Rc<RefCell<Octopus>>>> = Vec::new();

    // populate Octopus grid
    for line in src.lines() {
        let mut octopi_row: Vec<Rc<RefCell<Octopus>>> = Vec::new();
        
        for ch in line.chars() {
            octopi_row.push(Rc::new( RefCell::new ( Octopus { energy: ch.to_string().parse().unwrap(), flashed: false, adj_octopi: vec![] } )));
        }
        octopi_grid.push(octopi_row);
    }

    // iter over Octopus grid and populate Octopus::adj_octopi data
    // Assumption: Octopus grid is 10x10
    for row_idx in 0..10  {
        for col_idx in 0..10 {

            // three top cells
            if row_idx > 0 {
                // top cell
                octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx-1][col_idx]));

                // top-left cell
                if col_idx > 0 {
                    octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx-1][col_idx-1]));
                }

                // top-right cell
                if col_idx < 9 {
                    octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx-1][col_idx+1]));
                }
            }

            // two horizontal adj cells
            if col_idx > 0 {
                octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx][col_idx-1]));
            }
            if col_idx < 9 {
                octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx][col_idx+1]));
            }

            // three bottom cells
            if row_idx < 9 {
                // bottom cell
                octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx+1][col_idx]));

                // bottom-left cell
                if col_idx > 0 {
                    octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx+1][col_idx-1]));
                }

                // bottom-right cell
                if col_idx < 9 {
                    octopi_grid[row_idx][col_idx].borrow_mut().adj_octopi.push(Rc::clone(&octopi_grid[row_idx+1][col_idx+1]));
                }
            }
        }
    }

    for step in 0..STEPS {
        // Increment all by 1
        incr_all(&mut octopi_grid);

        let mut to_flash_stack: Vec<Rc<RefCell<Octopus>>> = get_all_to_flash(&mut octopi_grid);

        let mut step_flash_num: u64 = 0;


        // find all octopi to flash
        let mut cur_octopus: Rc<RefCell<Octopus>>;

        while !to_flash_stack.is_empty() {
            cur_octopus = to_flash_stack.pop().unwrap();

            cur_octopus.borrow_mut().energy = 0;
            cur_octopus.borrow_mut().flashed = true;

            step_flash_num += 1;

            for adj_octopus in cur_octopus.borrow_mut().adj_octopi.iter() {
                let mut adj_borrow_mut = adj_octopus.borrow_mut();

                // has a new flash been triggered, ignore octopi already marked to flash
                if adj_borrow_mut.energy <= 9 && !adj_borrow_mut.flashed {
                    adj_borrow_mut.energy += 1;

                    if adj_borrow_mut.energy == 10 {
                        adj_borrow_mut.flashed = true;
                        to_flash_stack.push(Rc::clone(adj_octopus));
                    }
                }
            }
        }

        reset_flashed_bool(&mut octopi_grid);

        if step_flash_num == 100 {
            println!("result: {}", step+1);
            
            break;
        }
        if step == (STEPS-1) {
            panic!("simultaneous flash not found");
        }
    }
}