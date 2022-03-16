use std::process;
use std::fs;
use std::env;

pub fn read_file_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let file: &str = match &args[..] {
        [_, file] => file,
        _ => {
            println!("Invalid arguments");
            process::exit(1);
        }
    };
    match fs::read_to_string(file) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Failed to read {} [{}]", file, err);
            process::exit(1);
        }
    }
}