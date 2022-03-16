use crate::util::file;


fn gen_rating(bin_str_filter_list: &mut Vec<(String, bool)>, bit_freqs: &mut Vec<u32>, is_oxygen_rating: bool) -> u32 {

    let mut is_one_most_common: bool;
    let mut target_bit_ch: char;

    let mut num_nums: u32 = bin_str_filter_list.len() as u32;
    let mut last_match: Option<String> = None;
    
    for bit_idx in 0..bit_freqs.len() {

        if num_nums < 2 { break }

        // Is 1 the most common or equally common value in this place?
        is_one_most_common = (bit_freqs[bit_idx] as f64) >= ((num_nums as f64) / 2.0);
        target_bit_ch = if (is_one_most_common && is_oxygen_rating) || ( !is_one_most_common && !is_oxygen_rating ) {
            '1'
        } else if (!is_one_most_common && is_oxygen_rating) || (is_one_most_common && !is_oxygen_rating) {
            '0'
        } else {
            panic!();
        };

        // iterate over binary strings and verify if can stay
        for bin_str_tuple in bin_str_filter_list.iter_mut().filter(|bin_tuple| { bin_tuple.1 }) {

            if bin_str_tuple.0.chars().nth(bit_idx).unwrap() == target_bit_ch {
                last_match = Some(bin_str_tuple.0.clone());
            } else {
                // Remove number from list of remaining numbers
                // Iterate over bits in number, and decrement relevant freqs in bit_freqs

                for (freq_bit_idx, bit) in bin_str_tuple.0.chars().enumerate() {
                    if bit == '1' {
                        bit_freqs[freq_bit_idx] -= 1;
                    }
                }

                bin_str_tuple.1 = false;
            }
        }

        num_nums = bin_str_filter_list.clone()
            .into_iter()
            .filter(|bin_tuple| { bin_tuple.1 })
            .count() as u32;

    }

    u32::from_str_radix(&last_match.unwrap(), 2).unwrap()
}

// TODO: Impl Binary Trie Approach
pub fn exec() {
    let src: String = file::read_file_arg();

    // left-to-right indexed bit freqs
    let mut bit_freqs: Vec<u32> = Vec::new();

    let mut bin_str_list: Vec<String> = Vec::new();

    for bin_str in src.lines() {
        for (digit_idx, digit) in bin_str.chars().enumerate() {
            if digit == '1' {

                if bit_freqs.len() <= digit_idx {
                    bit_freqs.resize(digit_idx+1, 0);
                }
                bit_freqs[digit_idx] += 1;

            }
        }

        bin_str_list.push(String::from(bin_str));
    }


    let bin_str_filter_list: Vec<(String, bool)> = bin_str_list.iter()
        .map(|bin_str| { (String::from(bin_str), true) })
        .collect();

    
    let oxygen_gen_rating = gen_rating(&mut bin_str_filter_list.clone(), &mut bit_freqs.clone(), true);
    let co2_scrubber_rating = gen_rating(&mut bin_str_filter_list.clone(), &mut bit_freqs.clone(), false);

    println!("oxygen generator rating: {:?}", oxygen_gen_rating);
    println!("CO2 scrubber rating: {:?}", co2_scrubber_rating);
    println!("life support rating: {:?}", (oxygen_gen_rating*co2_scrubber_rating))

}