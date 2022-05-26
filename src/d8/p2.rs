use std::collections::{ HashSet, HashMap };

// used for resolving three segment_idx pairs: [1, 3], [4, 6], [2, 5]
// to identify which character matches which segment_idx
fn resolve_pair(patterns_to_value: &[&str], pair_idx_chars: &(char, char), use_idx_zero: bool) -> (char, char) {

    let idx_to_use: usize = if use_idx_zero { 0 } else { 1 };

    if patterns_to_value[idx_to_use].contains(pair_idx_chars.0) && !patterns_to_value[idx_to_use].contains(pair_idx_chars.1) {
        (pair_idx_chars.1, pair_idx_chars.0)
    } else if !patterns_to_value[idx_to_use].contains(pair_idx_chars.0) && patterns_to_value[idx_to_use].contains(pair_idx_chars.1) {
        (pair_idx_chars.0, pair_idx_chars.1)
    } else {
        panic!("invalid case");
    }
}


pub fn exec(src: String) {

    // len 2 = 1
    // len 3 = 1
    // len 4 = 1
    // len 5 = 3
    // len 6 = 3
    // len 7 = 1

    /* Useful reference:
    let output_to_segment_map: Vec<Vec<u64>> = vec![
        vec![0, 1, 2, 4, 5, 6], // 0
        vec![2, 5], // 1
        vec![0, 2, 3, 4, 6], // 2
        vec![0, 2, 3, 5, 6], // 3
        vec![1, 2, 3, 5], // 4
        vec![0, 1, 3, 5, 6], // 5
        vec![0, 1, 3, 4, 5, 6], // 6
        vec![0, 2, 5], // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6], // 9
    ];
    */

    let segment_idx_to_value_map: Vec<u64> = vec![
        654210, // 0
        52, // 1
        64320, // 2
        65320, // 3
        5321, // 4
        65310, // 5
        654310, // 6
        520, // 7
        6543210, // 8
        653210, // 9
    ];

    /* Segment -> index mapping
        00
       1  2
       1  2
        33
       4  5
       4  5
        66
    */

    let mut data: Vec<(Vec<&str>, Vec<&str>)> = vec![];

    for line in src.lines() {
        let mut reached_output: bool = false;

        let mut patterns: Vec<&str> = Vec::new();
        let mut outputs: Vec<&str> = Vec::new();

        for token in line.split(' ') {

            if token == "|" {
                reached_output = true;
                continue;
            }

            if !reached_output {
                patterns.push(token);
            } else {
                outputs.push(token);
            }
        }
       data.push((patterns, outputs));
    }

    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut result: u64 = 0;

    for mut display in data.into_iter() {


        // sort patterns
        display.0.sort_by(|a, b| a.chars().count().partial_cmp(&b.chars().count()).unwrap());

        let mut char_possible_segments: HashMap<char, Vec<u64>> = HashMap::new();

        for ch in char_list.iter() {
            char_possible_segments.insert(*ch, Vec::new());
        }

        let mut prev_seen_chars: HashSet<char> = HashSet::new();
        let mut cur_ch: char;

        // chars that are in segment idx: [2, 5], [1, 3], [4, 6]
        let mut two_five_segment_idx_chars: (char, char) = ('0', '0');
        let mut one_three_segment_idx_chars: (char, char) = ('0', '0');
        let mut four_six_segment_idx_chars: (char, char) = ('0', '0');


        // outputs are irrelevant (duplicates) to mapping wires to segments
        // 1 --> 7 --> 4
        for (idx, pattern) in display.0[0..3].iter().enumerate() {
            let mut pattern_chars = pattern.chars();

            // 1
            if idx == 0 {
                cur_ch = pattern_chars.next().unwrap();
                prev_seen_chars.insert(cur_ch);
                char_possible_segments.insert(cur_ch, vec![2, 5]);
                two_five_segment_idx_chars.0 = cur_ch;

                cur_ch = pattern_chars.next().unwrap();
                prev_seen_chars.insert(cur_ch);
                char_possible_segments.insert(cur_ch, vec![2, 5]);
                two_five_segment_idx_chars.1 = cur_ch;

            }
            // 7
            else if idx == 1 {
                cur_ch = pattern_chars.find(|x| { !prev_seen_chars.contains(x) }).unwrap();
                char_possible_segments.insert(cur_ch, vec![0]);
            }
            // 4
            else if idx == 2 {
                cur_ch = pattern_chars.find(|x| { !prev_seen_chars.contains(x) }).unwrap();
                char_possible_segments.insert(cur_ch, vec![1, 3]);
                prev_seen_chars.insert(cur_ch);
                one_three_segment_idx_chars.0 = cur_ch;

                cur_ch = pattern_chars.find(|x| { !prev_seen_chars.contains(x) }).unwrap();
                char_possible_segments.insert(cur_ch, vec![1, 3]);
                prev_seen_chars.insert(cur_ch);
                one_three_segment_idx_chars.1 = cur_ch;
            }
        }

        // set last pair
        cur_ch = *char_possible_segments.iter().find(|(_ch, segment_idx_list)| { segment_idx_list.is_empty() }).unwrap().0;
        char_possible_segments.insert(cur_ch, vec![4, 6]);
        four_six_segment_idx_chars.0 = cur_ch;

        cur_ch = *char_possible_segments.iter().find(|(_ch, segment_idx_list)| { segment_idx_list.is_empty() }).unwrap().0;
        char_possible_segments.insert(cur_ch, vec![4, 6]);
        four_six_segment_idx_chars.1 = cur_ch;

        // After the first three unique length patterns (1 --> 7 --> 4), there will always be one solved letter (segment_idx = 0)
        // and 3 pairs of characters each with two identical possible segment_idx values
        // Out of length 5 patterns (3 patterns), all correct values can be derived
        // check which single pattern contains contains both letters:
        //     [4,6] --> 2
        //     [2,5] --> 3
        //     [1,3] --> 5
        //     Check the other two, segments solve based on which letter is missing 
        //         [4,6] --> char not found in other patterns = 4
        //         [1,3] --> char not found in other patterns = 1
        //         Now, check the pattern for 2 & 3, the char from [2,5] in both is segment_idx = 2
        //     all segment_idx values now correctly solved


        // correctly 1<->1 map three 5-length patterns to three possible segments (2, 3, 5)
        // [2_pattern, 3_pattern, 5_pattern]
        let mut patterns_to_value: Vec<&str> = vec![""; 3];

        for pattern in display.0[3..6].iter() {
            if pattern.contains(two_five_segment_idx_chars.0) && pattern.contains(two_five_segment_idx_chars.1) {
                patterns_to_value[1] = pattern;
            } else if pattern.contains(one_three_segment_idx_chars.0) && pattern.contains(one_three_segment_idx_chars.1) {
                patterns_to_value[2] = pattern;
            } else if pattern.contains(four_six_segment_idx_chars.0) && pattern.contains(four_six_segment_idx_chars.1) {
                patterns_to_value[0] = pattern;
            }
        }


        // Identify char for segment_idx = 4 --> Resolves [4,6] pair (both segments only found for value 2)
        let (segment_idx_four_ch, segment_idx_six_ch) = resolve_pair(&patterns_to_value, &four_six_segment_idx_chars, false);

        char_possible_segments.insert(segment_idx_four_ch, vec![4]);
        char_possible_segments.insert(segment_idx_six_ch, vec![6]);


        // Identify char for segment_idx = 1 --> Resolves [1,3] pair (both segments only found for value 3)
        let (segment_idx_one_ch, segment_idx_three_ch) = resolve_pair(&patterns_to_value, &one_three_segment_idx_chars, false);

        char_possible_segments.insert(segment_idx_one_ch, vec![1]);
        char_possible_segments.insert(segment_idx_three_ch, vec![3]);


        // Identify char for segment_idx = 2 --> Resolves [2,5] pair (both segments only found for value 5)
        let (segment_idx_five_ch, segment_idx_two_ch) = resolve_pair(&patterns_to_value, &two_five_segment_idx_chars, true);

        char_possible_segments.insert(segment_idx_five_ch, vec![5]);
        char_possible_segments.insert(segment_idx_two_ch, vec![2]);


        let mut display_output_sum: u64 = 0;

        let mut display_output_val: u64 = 0;
        let mut segment_idx_list: Vec<u64> = vec![];
        for (_output_idx, output) in display.1.iter().enumerate() {

            for ch in output.chars() {
                segment_idx_list.push(char_possible_segments.get(&ch).unwrap()[0]);
            }

            segment_idx_list.sort_unstable();
            segment_idx_list.reverse();
            
            // lookup value from segment_idx_to_value_map

            for segment_idx in segment_idx_list.iter() {
                display_output_val *= 10;
                display_output_val += segment_idx;
            }

            let val_to_add = segment_idx_to_value_map.iter().position(|x| *x == display_output_val).unwrap();

            display_output_sum *= 10;
            display_output_sum += val_to_add as u64;

            display_output_val = 0;
            segment_idx_list = vec![];

        }
        result += display_output_sum;
    }

    println!("result: {:?}", result);

}