use crate::util::file;

pub fn exec() {
    let src: String = file::read_file_arg();


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

    // number of segments for values:
    // 1 --> 2
    // 4 --> 4
    // 7 --> 3
    // 8 --> 7

    let mut output_val_freq: u64 = 0;

    for display in data.iter() {

        for output in display.1.iter() {

            if output.chars().count() == 2 ||
            output.chars().count() == 3 ||
            output.chars().count() == 4 ||
            output.chars().count() == 7 {
                output_val_freq += 1;
            }
        }
    }

    println!("{}", output_val_freq);


}