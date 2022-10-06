use std::collections::HashMap;

const STEPS: u64 = 10;


pub fn exec(src: &str, print: bool) {
    let mut src_lines = src.lines();

    let mut rule_lookup: HashMap<&str, char> = HashMap::new();

    let base: &str = src_lines.next().unwrap();

    src_lines.next().unwrap();
    
    for line in src_lines {
        let mut l_split = line.split(" -> ");
        let rule_pattern = l_split.next().unwrap();
        let rule_insert = l_split.next().unwrap().chars().next().unwrap();

        rule_lookup.insert(rule_pattern, rule_insert);
    }

    let mut base: Vec<char> = base.chars().collect();

    for _ in 0..STEPS {
        let mut new_base: Vec<&char> = Vec::new();
        for (base_idx, ch) in base[0..base.len()-1].iter().enumerate() {

            new_base.push(ch);

            let mut rule_pattern = String::from(*ch);
            rule_pattern.push(base[base_idx+1]);

            if let Some(ch_to_insert) = rule_lookup.get(rule_pattern.as_str()) {
                new_base.push(ch_to_insert);
            }
        }

        new_base.push(&base[base.len()-1]);
        base = new_base.into_iter().cloned().collect();
    }

    let mut ch_freq_map: HashMap<char, u16> = HashMap::new();

    for ch in base.iter() {
        if ch_freq_map.get(ch).is_none() { ch_freq_map.insert(*ch, 0); }
        let freq_ref = ch_freq_map.get_mut(ch).unwrap();
        *freq_ref += 1;
    }


    let mut min_freq: u16 = *ch_freq_map.get(&base[0]).unwrap();
    let mut max_freq: u16 = 0;

    for (_ch, freq) in ch_freq_map.iter() {
        if *freq < min_freq { min_freq = *freq; }
        if *freq > max_freq { max_freq = *freq; }
    }

    if print { println!("result: {}", max_freq - min_freq) }
}