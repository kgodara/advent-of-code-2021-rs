use std::collections::{ VecDeque, HashSet, HashMap, hash_map::{ Entry }, };

fn exec_program(consts: &(u8, i16, u8), inp: u8, z: &mut i32) {

    let (mut x, mut y, mut w): (i32, i32, i32) = (0, 0, inp as i32);

    // if z is divided by 1 (const.0 == 1):
    //     the number added to x is ALWAYS >= 10 (const.1 >= 10)
    //     -> x = 1 ( since x != to any single-digit 'w')
    //     -> y = 26
    //     -> z = (z*26) + (w+?) --> for first region, z = w + ?

    if consts.0 == 1 {
        *z *= 26;
        *z += w + (consts.2 as i32);
    } else if consts.0 == 26 {
        // if z is divided by 26 (const.0 == 26):
        //     the number added to x is ALWAYS <= 0 (const.1 <= 0)
        //     thus, x = 1 if (z%26) + const.1) != w
        x = if ((*z % 26) + (consts.1 as i32)) == w { 0 } else { 1 };
        y = if x == 0 { 1 } else { 26 };
        *z = (*z / 26) * y;
        y = (w + (consts.2 as i32)) * x;
        *z += y;

    } else {
        panic!("invalid const.0!");
    }
}


pub fn exec(src: String) {

    let mut inp_region_consts: Vec<(u8, i16, u8)> = vec![];

    // inp_region differences: (not counting inp)
    //     "div z (1|26)"
    //     "add x (-?\d+)"
    //     "add y (\d+)"

    let mut cur_consts: (u8, i16, u8) = (0, 0, 0);

    let mut prev_line: &str = "";

    for line in src.lines() {
        let mut args = line.split_ascii_whitespace();

        let i_type = args.next().unwrap();
        let a_arg = args.next().unwrap();
        let b_arg = args.next();

        // "div z (1|26)"
        if i_type == "div" && a_arg == "z" {
            cur_consts.0 = str::parse(b_arg.unwrap()).unwrap();
        }
        // "add x (-?\d+)"
        if i_type == "add" && a_arg == "x" && str::parse::<i16>(b_arg.unwrap()).is_ok() {
            cur_consts.1 = str::parse(b_arg.unwrap()).unwrap();
        }
        // "add y (\d+)"
        if prev_line == "add y w" {
            cur_consts.2 = str::parse(b_arg.unwrap()).unwrap();
            inp_region_consts.push(cur_consts);
            cur_consts = (0, 0, 0);
        }

        prev_line = line.trim();
    }

    println!("inp_region_consts: {:?}", inp_region_consts);

    // Approach: separate program by inp calls, generate results digit-by-digit,

    // store 2-tuple (z, digit_idx) -> max_prefix in HashMap
    // Note: x is set to 0 at start of every inp region, don't need to store
    // Note: y is set to 0 at start of every inp region, don't need to store
    // Note: w is set to inp at start of every inp region, don't need to store
    let mut state_set: HashMap<(i32, i8), u64> = HashMap::new();
    let mut state_queue: VecDeque<(i32, i8)> = VecDeque::new();

    let mut z: i32;

    let mut max_valid: u64 = 0;

    // init stack
    for digit in 1..10 {
        z = 0;

        exec_program(&inp_region_consts[0], digit, &mut z);

        // (z) are all unique for all digits for first inp region
        // no need to cmp
        state_set.insert((z, 1), digit as u64);
        state_queue.push_back((z, 1) );
    }

    let mut cur_digit_idx = 0;


    while !state_queue.is_empty() {
        let cur_state = state_queue.pop_front().unwrap();

        let digit_idx = cur_state.1;
        let new_digit_idx = cur_state.1+1;

        let prefix: u64 = *state_set.get(&(cur_state.0, cur_state.1)).unwrap();

        // execute all possible successions of cur_state
        for digit in 1..10 {
            z = cur_state.0.clone();
            let new_prefix = (prefix*10)+(digit as u64);

            exec_program(&inp_region_consts[digit_idx as usize], digit, &mut z);

            // 14-length number was just tested (cur_state.4 == 13), check validity & don't spawn children
            // valid if z == 0
            if new_digit_idx == 14 {
                if z == 0 {
                    max_valid = std::cmp::max(max_valid, new_prefix);
                }
                continue;
            }

            match state_set.entry((z, new_digit_idx)) {
                Entry::Occupied(entry) => {
                    *entry.into_mut() = std::cmp::max(new_prefix, *entry.get());
                },
                Entry::Vacant(entry) => {
                    entry.insert(new_prefix);
                    state_queue.push_back((z, new_digit_idx));
                },
            }
        }
    }

    println!("max_valid: {}", max_valid);
}