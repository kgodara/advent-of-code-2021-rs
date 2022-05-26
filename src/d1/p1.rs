use std::process;

pub fn exec(src: String) {

    let mut input_data: Vec<u16> = Vec::new();
    for num_str in src.split_whitespace() {
        input_data.push(num_str.parse::<u16>().unwrap());
    }

    if input_data.len() < 2 {
        println!("0");
        process::exit(1);
    }

    let mut prev: u16 = input_data[1];
    let mut num_increase: u16 = 0;
    
    for num in input_data[1..].iter() {
        if num > &prev {
            num_increase += 1;
        }
        prev = *num;
    }

    println!("result: {:?}", num_increase);
}