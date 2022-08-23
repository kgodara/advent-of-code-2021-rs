
pub fn exec(src: String) {

    let input_data: Vec<u16> = src.split_whitespace().map(|num_str| num_str.parse::<u16>().unwrap()).collect();

    println!("result: {}",
        input_data.windows(3)
            .zip(input_data[1..].windows(3))
            .map(|(prev_window, next_window)|
                if next_window.iter().sum::<u16>() > prev_window.iter().sum::<u16>() { 1 } else { 0 } )
            .sum::<u16>()
    );
}