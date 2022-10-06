
pub fn exec(src: &str, print: bool) {

    // 12 bit binary numbers
    let mut digit_freqs: Vec<u32> = vec![0; 12];

    let mut bin_num_count: u32 = 0;

    for bin_str in src.lines() {
        for (digit_idx, digit)in bin_str.bytes().enumerate() {
            if digit == b'1' {
                digit_freqs[digit_idx] += 1;
            }
        }
        bin_num_count += 1;
    }

    let mut gamma_rate: u64 = 0;
    let mut epsilon_rate: u64 = 0;

    for freq in digit_freqs {

        gamma_rate <<= 1;
        epsilon_rate <<= 1;

        // Is 1 the most common value in this place?
        if freq >= (bin_num_count / 2) {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
    }

    if print { println!("result: {:?}", gamma_rate*epsilon_rate) }
}