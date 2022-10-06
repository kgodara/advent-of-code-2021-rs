pub fn exec(src: &str, print: bool) {

    // NOTES:
    // Use compressed version of inp region

    // inp_region differences: (not counting inp)
    //     "div z (1|26)"
    //     "add x (-?\d+)"
    //     "add y (\d+)"


    // if z is divided by 1 (const.0 == 1):
    //     the number added to x is ALWAYS >= 10 (const.1 >= 10)
    //     -> x = 1 ( since x != to any single-digit 'w')
    //     -> y = 26
    //     -> z = (z*26) + (w+?) --> for first region, z = w + ?
    //     

    // region #2, z = w + ?
    // NEXT REGION (z / 1):
    // -> x = 1
    // -> y = 26
    // -> z = (z*26) + (w+?)

    // (z/1) regions store w + ? (+ some multiple of 26) -->
    //     successive (z/1) regions keep storing (w+?) values in different (z*26) levels
    // (z/26) regions start by extracting the latest (w+?) into x and pulling z down a level (z/26)
    //     if prev_(w+?)+? == w: z = 0
    //         (AKA) x == 0
    //     else (x == 1 --> y = 26): z = z * 26 (w + ?)

    // Thus:
    //     (z/1) regions = (w+?) STORE regions
    //     (z/26) regions = (w+?) EXTRACT regions IF (?["add x ?"] + (prev_w["add y ?"]+?)) == w
    //         else z/26 regions will function as STORE regions

    //     all (z/26) regions must function as EXTRACT regions (to match provided STORE regions)
    //     in order for z=0 at the end of the program
    //     Thus, the 2 inputs for a (z/26) region and the latest (z/1) region are linked

    // If we have (z/26) following (z/1):
    // x = 1 if (consts.1 + sum of previous 'w's stored in z) == inp
    // range of acceptable inputs must satisfy:
    //     w - prev_w = prev_?["add y ?"] + ?["add x ?"]
    // goal is to (max|min)imize prev_w(earlier input = higher magnitude) then w

    // inp_region differences: (not counting inp)
    //     "div z (1|26)"
    //     "add x (-?\d+)"
    //     "add y (\d+)"

    let mut inp_region_consts: Vec<(u8, i16, i16)> = vec![];



    let mut cur_consts: (u8, i16, i16) = (0, 0, 0);

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

    // println!("inp_region_consts: {:?}", inp_region_consts);

    let mut store_region_stack: Vec<(u8, i16)> = vec![];

    // ((store_digit_idx, extract_digit_idx), range)
    let mut store_extract_pairs: Vec<((u8, u8), i16)> = vec![];

    for (region_idx, region_consts) in inp_region_consts.iter().enumerate() {
        // is STORE region
        if region_consts.0 == 1 {
            // num added to x MUST always be >= 10,
            // otherwise x could be set to 0,
            // making STORE region function as EXTRACT region
            assert!(region_consts.1 >= 10);

            // w + ?["add y ?"]
            // (["add x ?"] value is meaningless here)
            store_region_stack.push((region_idx as u8, region_consts.2));
        } else {
            // is EXTRACT region
            
            // num added to x always <= 0,
            assert!(region_consts.1 <= 0);

            let linked_store_region: (u8, i16) = store_region_stack.pop().unwrap();
            // ((store_region_idx, extract_region_idx), (store_region["add y ?"] + extract_region["add x ?"]))
            store_extract_pairs.push(((linked_store_region.0, region_idx as u8), linked_store_region.1 + region_consts.1));
        }
    }

    let mut result: Vec<u8> = vec![0; 14];

    // region_pair: ((store_region_idx, extract_region_idx), (store_region["add y ?"] + extract_region["add x ?"]))
    for region_pair in store_extract_pairs.iter() {
        // find min pair
        let mut min_lower_bound = 0;
        // find lowest lower-bound for range of two single-digit values separated by (region_pair.1)
        for lower_bound in (1..10).rev() {
            // NOTE: not >= 0, since 0 digits not allowed
            if (lower_bound + region_pair.1) > 0 && (lower_bound + region_pair.1) < 10 { min_lower_bound = lower_bound; }
        }
        result[region_pair.0.1 as usize] = (min_lower_bound + region_pair.1) as u8;
        result[region_pair.0.0 as usize] = min_lower_bound as u8;
    }
    


    if print { 
        println!("result: {:?}", str::parse::<u64>(
            &result
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .concat()
            ).unwrap()
        )
    }

}