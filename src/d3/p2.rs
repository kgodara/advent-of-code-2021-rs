
fn gen_rating(mut bin_nums: Vec<u16>, mut bit_freqs: Vec<u32>, is_oxygen_rating: bool) -> u16 {

    let mut is_one_most_common: bool;
    let mut target_bit_val: u8;

    let mut bin_num_count: u32 = bin_nums.len() as u32;
    let mut last_match: u16 = 0;
    
    for bit_idx in 0..bit_freqs.len() {

        if bin_num_count < 2 { break }

        // Is 1 the most common or equally common value in this place?
        is_one_most_common = (bit_freqs[bit_idx] as f64) >= ((bin_num_count as f64) / 2.0);

        target_bit_val = if (is_one_most_common && is_oxygen_rating) || ( !is_one_most_common && !is_oxygen_rating ) {
                1
            } else if (!is_one_most_common && is_oxygen_rating) || (is_one_most_common && !is_oxygen_rating) {
                0
            } else {
                panic!();
        };

        bin_nums.retain(|num| {

            // num >> bit_idx: right -> left traversal
            // num >> (bit_freqs.len()-1-bit_idx): left -> right traversal

            if ((num >> (bit_freqs.len() - 1 - bit_idx)) & 1) == target_bit_val as u16 {
                last_match = *num;
                true
            } else {

                // Iterate over bits in num, and decrement relevant freqs in bit_freqs
                // 12-bit nums: iterate over bits right -> left

                let mut freq_bit_idx = 11;
                let mut num_decr = *num;

                while num_decr > 0 {
                    bit_freqs[freq_bit_idx] -= (num_decr & 1) as u32;
                    num_decr >>= 1;
                    freq_bit_idx -= 1;
                }
                false
            }
        });

        bin_num_count = bin_nums.len() as u32;

    }

    last_match
}

pub fn exec(src: String) {

    // left-to-right indexed bit freqs
    // 12-bit binary nums
    let mut bit_freqs: Vec<u32> = vec![0; 12];

    let mut bin_nums: Vec<u16> = Vec::new();

    for bin_str in src.lines() {
        for (digit_idx, digit) in bin_str.bytes().enumerate() {
            if digit == b'1' {
                bit_freqs[digit_idx] += 1;
            }
        }
        bin_nums.push(u16::from_str_radix(bin_str, 2).unwrap());
    }

    let oxygen_gen_rating = gen_rating(bin_nums.clone(), bit_freqs.clone(), true);

    let co2_scrubber_rating = gen_rating(bin_nums.clone(), bit_freqs.clone(), false);

    println!("result: {:?}", ((oxygen_gen_rating as u32)*(co2_scrubber_rating as u32)));
}