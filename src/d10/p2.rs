pub fn exec(src: &str, print: bool) {

    let open_chars: Vec<char> = vec!['(', '[', '{', '<'];
    let close_chars: Vec<char> = vec![')', ']', '}', '>'];

    let autocomplete_scores: Vec<u64> = vec![1, 2, 3, 4];

    let mut score_list: Vec<u64> = Vec::new();

    for line in src.lines() {

        let mut open_ch_stack: Vec<usize> = Vec::new();
        let mut line_is_corrupted: bool = false;

        for ch in line.chars() {
            // opening char
            if let Some(open_idx) = open_chars.iter().position(|open_ch| *open_ch == ch) {
                open_ch_stack.push(open_idx);
            }
            // closing char
            else {
                // line is corrupted
                if open_ch_stack.is_empty() {
                    line_is_corrupted = true;
                    break;
                }

                let close_ch_idx: usize = close_chars.iter().position(|close_ch| *close_ch == ch).unwrap();
                
                // line is corrupted
                if close_ch_idx != open_ch_stack.pop().unwrap() {
                    line_is_corrupted = true;
                    break;
                }
            }
        }

        let mut line_score: u64 = 0;

        // open_ch_stack now contains all open chars that need to be closed
        while !open_ch_stack.is_empty() && !line_is_corrupted {
            line_score *= 5;
            line_score += autocomplete_scores[open_ch_stack.pop().unwrap()];
        }
        if !line_is_corrupted {
            score_list.push(line_score);
        }
    }

    score_list.sort_unstable();

    // Assumption: there is an odd number of incomplete lines
    if print { println!("result: {}", score_list[score_list.len()/2]) }
}