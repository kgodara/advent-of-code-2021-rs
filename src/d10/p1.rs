pub fn exec(src: String) {

    let open_chars: Vec<char> = vec!['(', '[', '{', '<'];
    let close_chars: Vec<char> = vec![')', ']', '}', '>'];

    let corrupt_char_score: Vec<u64> = vec![ 3, 57, 1197, 25137];

    let mut score_sum: u64 = 0;

    for line in src.lines() {

        let mut open_ch_stack: Vec<usize> = Vec::new();
        let mut corrupt_char_idx: Option<usize> = None;

        for ch in line.chars() {
            // opening char
            if let Some(open_idx) = open_chars.iter().position(|open_ch| *open_ch == ch) {
                open_ch_stack.push(open_idx);
            } 
            // closing char
            else {
                if open_ch_stack.is_empty() {
                    break;
                }

                let close_ch_idx: usize = close_chars.iter().position(|close_ch| *close_ch == ch).unwrap();
                
                if close_ch_idx != open_ch_stack.pop().unwrap() {
                    corrupt_char_idx = Some(close_ch_idx);
                    break;
                }
                
            }
        }

        if let Some(corrupt_char_idx_val) = corrupt_char_idx {
            score_sum += corrupt_char_score[corrupt_char_idx_val];
        }
    }

    println!("result: {}", score_sum);
}