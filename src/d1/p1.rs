
pub fn exec(src: &str, print: bool) {

    let input_data: Vec<u16> = src.split_whitespace().map(|num_str| num_str.parse::<u16>().unwrap()).collect();

    if print { println!("result: {}", input_data.windows(2).map(|pair| if pair[1] > pair[0] { 1 } else { 0 }).sum::<u16>()) }
}