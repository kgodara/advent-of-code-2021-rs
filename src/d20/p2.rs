const NUM_ITER: usize = 50;
const GRID_EDGE_SIZE: usize = 300;
const LEFT_BOUND: usize = ((GRID_EDGE_SIZE) / 2) - 50;
const RIGHT_BOUND: usize = ((GRID_EDGE_SIZE) / 2) + 50;

// TODO: Fix this cringe solution

// (row, col)
fn count_light_pixels(pixels: &[Vec<bool>]) -> u64 {
    let mut result: u64 = 0;
    for row_idx in (LEFT_BOUND - NUM_ITER)..(RIGHT_BOUND + NUM_ITER) {
        for col_idx in (LEFT_BOUND - NUM_ITER)..(RIGHT_BOUND + NUM_ITER) {
            result += if pixels[row_idx as usize][col_idx as usize] { 1 } else { 0 };
        }
    }
    result
}

pub fn exec(src: &str, print: bool) {
    let mut lines = src.lines();

    // true = light, false = dark
    let image_enhance_algo: Vec<bool> = lines.next()
        .unwrap()
        .chars()
        .map(|ch| {
            match ch {
                '#' => { true },
                '.' => { false },
                _ => {panic!("invalid image enhancement algorithm char")}
            }
        })
        .collect();


    if image_enhance_algo.len() != 512 {
        panic!("Invalid image enhancement algorithm! Expected 512 chars, got {}!", image_enhance_algo.len())
    }

    lines.next();

    let mut image_prev: Vec<Vec<bool>> = vec![vec![false; GRID_EDGE_SIZE]; GRID_EDGE_SIZE];
    let mut image_next: Vec<Vec<bool>> = vec![vec![false; GRID_EDGE_SIZE]; GRID_EDGE_SIZE];


    // init original image and determine bounds
    for (row_idx, line) in lines.enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            image_prev[row_idx+LEFT_BOUND][col_idx+LEFT_BOUND] = ch == '#';
        }
    }

    // generate 9-pixel (3x3) window given coord
    let gen_window = |coord: &(usize, usize), window_coords: &mut Vec<(i32, i32)>| {
        let mut count: usize = 0;
        for w_row_idx in -1..2 {
            for w_col_idx in -1..2 {
                window_coords[count] = ((coord.0 as i32) + w_row_idx, (coord.1 as i32) + w_col_idx);
                count += 1;
            }
        }
    };

    let mut window_coords: Vec<(i32, i32)> = vec![(0, 0); 9];

    // exec enhancement
    for _ in 0..NUM_ITER {
        for row_idx in 1..(image_prev.len()-1) {
            for col_idx in 1..(image_prev[0].len()-1) {
                // create 9-bit binary number
                gen_window(&(row_idx, col_idx), &mut window_coords);

                let mut lookup_val: u16 = 0;
                // println!("window_coords: {:?}", window_coords);

                // generate enhancement lookup val
                for window_coord in window_coords.iter() {

                    // Equivalent:
                    // lookup_val <<= 1;
                    lookup_val *= 2;
                    lookup_val += if image_prev[window_coord.0 as usize][window_coord.1 as usize] { 1 } else { 0 };
                }

                let new_pixel: bool = image_enhance_algo[lookup_val as usize];
            
                image_next[row_idx][col_idx] = new_pixel;
            }
        }

        image_prev = image_next;
        image_next = vec![vec![false; GRID_EDGE_SIZE]; GRID_EDGE_SIZE];
    }

    if print { println!("result: {}\n", count_light_pixels(&image_prev)) }
}