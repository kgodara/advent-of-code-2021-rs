// Assumption: Provided inputs provide rules to cover every possible permutation of possible characters

use std::collections::HashMap;

const STEPS: u64 = 40;


// Rust String Interner: https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html
#[derive(Default)]
pub struct Interner {
    pub map: HashMap<String, u8>,
    pub vec: Vec<String>,
}

impl Interner {
    pub fn intern(&mut self, name: &str) -> u8 {
        if let Some(&idx) = self.map.get(name) {
            return idx;
        }
        let idx = self.map.len() as u8;
        self.map.insert(name.to_owned(), idx);
        self.vec.push(name.to_owned());

        idx
    }

    pub fn lookup(&self, idx: u8) -> &str {
        self.vec[idx as usize].as_str()
    }
}


pub fn exec(src: String) {
    let mut src_lines = src.lines();

    let mut pair_interner: Interner = Interner::default();
    let mut rule_pat_to_children: HashMap<u8, (u8, u8)> = HashMap::default();

    let base: &str = src_lines.next().unwrap();

    src_lines.next().unwrap();
    
    for line in src_lines {
        let mut l_split = line.split(" -> ");

        let rule_pat_str: &str = l_split.next().unwrap();
        let rule_insert: char = l_split.next().unwrap().chars().next().unwrap();

    
        let rule_pat_interned: u8 = pair_interner.intern(rule_pat_str);

        let mut rule_pat_chars = rule_pat_str.chars();

        let rule_pat_child_one: &str = &format!("{}{}", rule_pat_chars.next().unwrap(), rule_insert);
        let rule_pat_child_two: &str = &format!("{}{}", rule_insert, rule_pat_chars.next().unwrap());

        rule_pat_to_children.insert(rule_pat_interned,
            (pair_interner.intern(rule_pat_child_one), pair_interner.intern(rule_pat_child_two))
        );
    }

    // instantiate with len == str_to_intern.len() -->
    //     this is number of unique pairs (all of which should have been seen in rules)
    let mut intern_freq_map: Vec<u64> = vec![0; pair_interner.map.len()];
    let mut intern_freq_map_temp: Vec<u64>;

    // these will be used to track which pairs ultimately end up at each edge of the final str
    // needed to correctly calculate final char freqs
    // (since non-edge chars are always shared by 2 pairs, generally can do freq/2,
    // but chars with > 0 occurences on edges need to use (freq-(1|2))/2)
    let mut left_edge_pair: u8 = 0;
    let mut right_edge_pair: u8 = 0;


    // convert base str to interned form
    let base_chars: Vec<char> = base.chars().collect();
    for base_ch_idx in 0..(base_chars.len()-1) {
        if base_ch_idx == 0 {
            left_edge_pair = pair_interner.intern(&format!("{}{}", base_chars[base_ch_idx], base_chars[base_ch_idx+1]));
        }
        if base_ch_idx == (base_chars.len()-2) {
            right_edge_pair = pair_interner.intern(&format!("{}{}", base_chars[base_ch_idx], base_chars[base_ch_idx+1]));
        }

        intern_freq_map[pair_interner.intern(&format!("{}{}", base_chars[base_ch_idx], base_chars[base_ch_idx+1])) as usize] += 1;
    }

    let mut pair_interned_freq: u64;

    for _step in 0..STEPS {

        intern_freq_map_temp = intern_freq_map.clone();

        // for all pairs possibly in str at this step
        for rule_pat_interned in 0..intern_freq_map.len() {
            pair_interned_freq = intern_freq_map[rule_pat_interned];

            // is this pair present in str at this step
            if pair_interned_freq > 0 {

                let (child_one, child_two) = rule_pat_to_children.get(&(rule_pat_interned as u8)).unwrap();

                // old pair is removed
                intern_freq_map_temp[rule_pat_interned] -= pair_interned_freq;

                // two new pairs are inserted
                intern_freq_map_temp[*child_one as usize] += pair_interned_freq;
                intern_freq_map_temp[*child_two as usize] += pair_interned_freq;

                // if old pair was on left edge
                if left_edge_pair == (rule_pat_interned as u8) {
                    // println!("updating left_edge_pair");
                    left_edge_pair = *child_one;
                }
                // if old pair was on right edge
                if right_edge_pair == (rule_pat_interned as u8) {
                    // println!("updating right_edge_pair");
                    right_edge_pair = *child_two;
                }
            }
        }
        intern_freq_map = intern_freq_map_temp.clone();
    }

    let left_ch: char = pair_interner.lookup(left_edge_pair).chars().next().unwrap();
    let right_ch: char = pair_interner.lookup(right_edge_pair).chars().last().unwrap();

    // determine char freqs:
    //     lookup original pairs (pair_interner.lookup()) for each element in intern_freq_map
    //     get both chars from pair
    //     add (pair_freq) to ch_freqs for each char
    let mut ch_freqs: HashMap<char, u64> = HashMap::default();


    for (pair_interned, pair_freq) in intern_freq_map.iter().enumerate() {
        let mut pair_str_chars = pair_interner.lookup(pair_interned as u8).chars();
        *ch_freqs.entry(pair_str_chars.next().unwrap()).or_insert(0) += pair_freq;
        *ch_freqs.entry(pair_str_chars.next().unwrap()).or_insert(0) += pair_freq;   
    }


    let mut min_freq: u64 = u64::MAX;
    let mut max_freq: u64 = 0;
    let mut num_edge_chars: u8;

    for (idx, (k, v)) in ch_freqs.iter().enumerate() {

        num_edge_chars = 0;
        if *k == left_ch {
            num_edge_chars += 1;
        }
        if *k == right_ch {
            num_edge_chars += 1;
        }

        // pairs result in each char being represented twice in freq counts,
        // but chars on edges are only represented once
        let ch_freq: u64 = ((*v-(num_edge_chars as u64)) / 2) + (num_edge_chars as u64);

        if idx == 0 {
            min_freq = ch_freq;
            max_freq = ch_freq;
        } else {
            if ch_freq < min_freq {
                min_freq = ch_freq;
            }
            if ch_freq > max_freq {
                max_freq = ch_freq;
            }
        }
    }

    println!("result: {}", max_freq - min_freq);
}