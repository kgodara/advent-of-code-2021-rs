#![allow(dead_code)]
#![feature(array_windows)]

use clap::Parser;

mod d1 { pub mod p1; pub mod p2; }
mod d2 { pub mod p1; pub mod p2; }
mod d3 { pub mod p1; pub mod p2; }
mod d4 { pub mod p1; pub mod p2; }
mod d5 { pub mod p1; pub mod p2; }
mod d6 { pub mod p1; pub mod p2; }
mod d7 { pub mod p1; pub mod p2; }
mod d8 { pub mod p1; pub mod p2; }
mod d9 { pub mod p1; pub mod p2; }
mod d10 { pub mod p1; pub mod p2; }
mod d11 { pub mod p1; pub mod p2; }
mod d12 { pub mod p1; pub mod p2; }
mod d13 { pub mod p1; pub mod p2; }
mod d14 { pub mod p1; pub mod p2; }
mod d15 { pub mod p1; pub mod p2; }
mod d16 { pub mod p1; pub mod p2; }
mod d17 { pub mod p1; pub mod p2; }
mod d18 { pub mod p1; pub mod p2; }
mod d19 { pub mod p1; pub mod p2; }
mod d20 { pub mod p1; pub mod p2; }
mod d21 { pub mod p1; pub mod p2; }
mod d22 { pub mod p1; pub mod p2; }
mod d24 { pub mod p1; pub mod p2; }
mod d23 { pub mod p1; pub mod p2; }
mod d25 { pub mod p1; }

mod util;

/// Execute AoC puzzle solution
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Day to exec: (1, ..., 25)
    #[clap(short, long)]
    day: u8,

    /// Day part to exec: (1, 2)
    #[clap(short, long)]
    part: u8,

    /// Input file suffix: "input/d{DAY}{SUFFIX}.txt" (for additional inputs)
    #[clap(short, long, default_value = "")]
    suffix: String,
}

enum DayPart {
    One,
    Two,
}

// #[]
fn main() {

    let args = Args::parse();

    let part: DayPart = match args.part {
        1 => { DayPart::One },
        2 => { DayPart::Two },
        _ => { panic!("Invalid day part - possible values: [1, 2]") }
    };

    let src: String = util::file::read_file(&format!("input/d{}{}.txt", args.day, args.suffix));

    use std::time::Instant;
    let now = Instant::now();

    match args.day {
        1 => {
            match part {
                DayPart::One => { d1::p1::exec(&src, true) },
                DayPart::Two => { d1::p2::exec(&src, true) },
            }
        },
        2 => {
            match part {
                DayPart::One => { d2::p1::exec(&src, true) },
                DayPart::Two => { d2::p2::exec(&src, true) },
            }
        },
        3 => {
            match part {
                DayPart::One => { d3::p1::exec(&src, true) },
                DayPart::Two => { d3::p2::exec(&src, true) },
            }
        },
        4 => {
            match part {
                DayPart::One => { d4::p1::exec(&src, true) },
                DayPart::Two => { d4::p2::exec(&src, true) },
            }
        },
        5 => {
            match part {
                DayPart::One => { d5::p1::exec(&src, true) },
                DayPart::Two => { d5::p2::exec(&src, true) },
            }
        },
        6 => {
            match part {
                DayPart::One => { d6::p1::exec(&src, true) },
                DayPart::Two => { d6::p2::exec(&src, true) },
            }
        },
        7 => {
            match part {
                DayPart::One => { d7::p1::exec(&src, true) },
                DayPart::Two => { d7::p2::exec(&src, true) },
            }
        },
        8 => {
            match part {
                DayPart::One => { d8::p1::exec(&src, true) },
                DayPart::Two => { d8::p2::exec(&src, true) },
            }
        },
        9 => {
            match part {
                DayPart::One => { d9::p1::exec(&src, true) },
                DayPart::Two => { d9::p2::exec(&src, true) },
            }
        },
        10 => {
            match part {
                DayPart::One => { d10::p1::exec(&src, true) },
                DayPart::Two => { d10::p2::exec(&src, true) },
            }
        },
        11 => {
            match part {
                DayPart::One => { d11::p1::exec(&src, true) },
                DayPart::Two => { d11::p2::exec(&src, true) },
            }
        },
        12 => {
            match part {
                DayPart::One => { d12::p1::exec(&src, true) },
                DayPart::Two => { d12::p2::exec(&src, true) },
            }
        },
        13 => {
            match part {
                DayPart::One => { d13::p1::exec(&src, true) },
                DayPart::Two => { d13::p2::exec(&src, true) },
            }
        },
        14 => {
            match part {
                DayPart::One => { d14::p1::exec(&src, true) },
                DayPart::Two => { d14::p2::exec(&src, true) },
            }
        },
        15 => {
            match part {
                DayPart::One => { d15::p1::exec(&src, true) },
                DayPart::Two => { d15::p2::exec(&src, true) },
            }
        },
        16 => {
            match part {
                DayPart::One => { d16::p1::exec(&src, true) },
                DayPart::Two => { d16::p2::exec(&src, true) },
            }
        },
        17 => {
            match part {
                DayPart::One => { d17::p1::exec(&src, true) },
                DayPart::Two => { d17::p2::exec(&src, true) },
            }
        },
        18 => {
            match part {
                DayPart::One => { d18::p1::exec(&src, true) },
                DayPart::Two => { d18::p2::exec(&src, true) },
            }
        },
        19 => {
            match part {
                DayPart::One => { d19::p1::exec(&src, true) },
                DayPart::Two => { d19::p2::exec(&src, true) },
            }
        },
        20 => {
            match part {
                DayPart::One => { d20::p1::exec(&src, true) },
                DayPart::Two => { d20::p2::exec(&src, true) },
            }
        },
        21 => {
            match part {
                DayPart::One => { d21::p1::exec(&src, true) },
                DayPart::Two => { d21::p2::exec(&src, true) },
            }
        },
        22 => {
            match part {
                DayPart::One => { d22::p1::exec(&src, true) },
                DayPart::Two => { d22::p2::exec(&src, true) },
            }
        },
        23 => {
            match part {
                DayPart::One => { d23::p1::exec(&src, true) },
                DayPart::Two => { d23::p2::exec(&src, true) },
            }
        }
        24 => {
            match part {
                DayPart::One => { d24::p1::exec(&src, true) },
                DayPart::Two => { d24::p2::exec(&src, true) },
            }
        }
        25 => {
            match part {
                DayPart::One => { d25::p1::exec(&src, true) },
                DayPart::Two => { unimplemented!() },
            }
        }

        _ => {panic!("Invalid day")}
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}