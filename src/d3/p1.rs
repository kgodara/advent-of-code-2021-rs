use std::cmp::Ordering;
use std::collections::BinaryHeap;

use std::collections::HashMap;
use crate::util::file;

// Use a HashMap to store digit frequencies, could use a vec as well
// Note: Handles input with different length binary numbers

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


pub fn exec() {
    let src: String = file::read_file_arg();

    // left-to-right indexed digit freqs
    let mut digit_freqs: HashMap<u32, u32> = HashMap::new();
    let mut num_nums: u32 = 0;

    for bin_str in src.lines() {
        for (digit_idx, digit) in bin_str.chars().enumerate() {
            if digit == '1' {
                *digit_freqs.entry(digit_idx as u32).or_insert(0) += 1;
            }
        }
        num_nums += 1;
    }

    let mut heap: BinaryHeap<DigitFreq> = BinaryHeap::new();

    let mut digit_idx: u32 = 0;

    let mut gamma_rate: u64 = 0;
    let mut epsilon_rate: u64 = 0;

    // sort frequences on min-heap
    while let Some(freq) = digit_freqs.get(&digit_idx) {
        heap.push(DigitFreq { idx: digit_idx, freq: *freq });
        digit_idx += 1;
    }

    while let Some(digit_freq) = heap.pop() {

        gamma_rate <<= 1;
        epsilon_rate <<= 1;

        // Is 1 the most common value in this place?
        if digit_freq.freq >= (num_nums / 2) {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
    }

    println!("{:?}", gamma_rate*epsilon_rate);
}