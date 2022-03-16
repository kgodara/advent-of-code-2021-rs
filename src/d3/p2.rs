use std::cmp::Ordering;
use std::collections::BinaryHeap;

use std::collections::HashMap;
use crate::util::file;


// Use a HashMap to store digit frequencies, could use a vec as well
// Handles input with different length binary numbers

#[derive(Eq, Debug)]
struct DigitFreq {
    idx: u32,
    freq: u32,
}

impl Ord for DigitFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.idx.cmp(&self.idx)
    }
}

impl PartialOrd for DigitFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.idx.partial_cmp(&self.idx)
    }
}

impl PartialEq for DigitFreq {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

// TODO: Impl Binary Trie Approach
pub fn exec() {
    let src: String = file::read_file_arg();


    // left-to-right indexed digit freqs
    let mut digit_freqs: HashMap<u32, u32> = HashMap::new();
    let mut num_nums: u32 = 0;

    let mut bin_str_list: Vec<String> = Vec::new();

    for bin_str in src.lines() {
        for (digit_idx, digit) in bin_str.chars().enumerate() {
            if digit == '1' {
                *digit_freqs.entry(digit_idx as u32).or_insert(0) += 1;
            }
        }

        bin_str_list.push(String::from(bin_str));
        num_nums += 1;
    }


    /*
    let mut base_ten_nums: Vec<u32> = bin_str_list.iter()
        .map(|bin_str| { u32::from_str_radix(bin_str, 2).unwrap() } )
        .collect();

    println!("base_ten_nums: {:?}", base_ten_nums);
    */

    let mut heap: BinaryHeap<DigitFreq> = BinaryHeap::new();

    let mut digit_idx: u32 = 0;

    // sort frequences on min-heap
    while let Some(freq) = digit_freqs.get(&digit_idx) {
        heap.push(DigitFreq { idx: digit_idx, freq: *freq });
        digit_idx += 1;
    }

    let mut last_match: Option<String> = None;

    let mut place_idx: usize = 0;

    let mut cur_digit_freq: DigitFreq;

    let mut is_one_most_common: bool;

    let mut bin_str_filter_list: Vec<(String, bool)> = bin_str_list.iter()
        .map(|bin_str| { (String::from(bin_str), true) })
        .collect();

    // oxygen generator rating
    // keep only numbers with MOST COMMON value in position
    //     if equally common pick those with 1
    loop {

        if let Some(digit_freq) = heap.pop() {
            // Is 1 the most common or equally common value in this place?
            is_one_most_common = (digit_freq.freq >= (num_nums / 2));
            println!("is_one_most_common: {:?}", is_one_most_common);

            let temp: Vec<(String, bool)> = bin_str_filter_list.clone().into_iter().filter(|bin_tuple| { bin_tuple.1 }).collect();
            println!("Remaining bin_nums: {:?}", temp);
            // iterate over binary strings and verify if can stay
            for (idx, bin_str_tuple) in bin_str_filter_list.iter_mut().filter(|bin_tuple| { bin_tuple.1 }).enumerate() {
                // println!("bin_str_tuple.0.chars().len(): {:?}", bin_str_tuple.0.chars().count());
                // println!("place_idx: {:?}\n", place_idx);
                if bin_str_tuple.0.chars().nth(place_idx).unwrap() == (if is_one_most_common { '1' } else { '0' }) {
                    last_match = Some(bin_str_tuple.0.clone());
                } else {
                    bin_str_tuple.1 = false;
                }
            }
        } else {
            break;
        }
        // println!("place_val: {:?}", bin_str_list.get(last_match_idx.unwrap() as usize));
        place_idx += 1;
    }

    println!("last_match: {:?}", last_match);

}