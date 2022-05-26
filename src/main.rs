#![allow(dead_code)]

use clap::Parser;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;

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
                DayPart::One => { d1::p1::exec(src) },
                DayPart::Two => { d1::p2::exec(src) },
            }
        },
        2 => {
            match part {
                DayPart::One => { d2::p1::exec(src) },
                DayPart::Two => { d2::p2::exec(src) },
            }
        },
        3 => {
            match part {
                DayPart::One => { d3::p1::exec(src) },
                DayPart::Two => { d3::p2::exec(src) },
            }
        },
        4 => {
            match part {
                DayPart::One => { d4::p1::exec(src) },
                DayPart::Two => { d4::p2::exec(src) },
            }
        },
        5 => {
            match part {
                DayPart::One => { d5::p1::exec(src) },
                DayPart::Two => { d5::p2::exec(src) },
            }
        },
        6 => {
            match part {
                DayPart::One => { d6::p1::exec(src) },
                DayPart::Two => { d6::p2::exec(src) },
            }
        },
        7 => {
            match part {
                DayPart::One => { d7::p1::exec(src) },
                DayPart::Two => { d7::p2::exec(src) },
            }
        },
        8 => {
            match part {
                DayPart::One => { d8::p1::exec(src) },
                DayPart::Two => { d8::p2::exec(src) },
            }
        },
        9 => {
            match part {
                DayPart::One => { d9::p1::exec(src) },
                DayPart::Two => { d9::p2::exec(src) },
            }
        },
        10 => {
            match part {
                DayPart::One => { d10::p1::exec(src) },
                DayPart::Two => { d10::p2::exec(src) },
            }
        },
        11 => {
            match part {
                DayPart::One => { d11::p1::exec(src) },
                DayPart::Two => { d11::p2::exec(src) },
            }
        },
        12 => {
            match part {
                DayPart::One => { d12::p1::exec(src) },
                DayPart::Two => { d12::p2::exec(src) },
            }
        },
        13 => {
            match part {
                DayPart::One => { d13::p1::exec(src) },
                DayPart::Two => { d13::p2::exec(src) },
            }
        },
        14 => {
            match part {
                DayPart::One => { d14::p1::exec(src) },
                DayPart::Two => { d14::p2::exec(src) },
            }
        },
        15 => {
            match part {
                DayPart::One => { d15::p1::exec(src) },
                DayPart::Two => { d15::p2::exec(src) },
            }
        },
        16 => {
            match part {
                DayPart::One => { d16::p1::exec(src) },
                DayPart::Two => { d16::p2::exec(src) },
            }
        },
        17 => {
            match part {
                DayPart::One => { d17::p1::exec(src) },
                DayPart::Two => { d17::p2::exec(src) },
            }
        },
        18 => {
            match part {
                DayPart::One => { d18::p1::exec(src) },
                DayPart::Two => { d18::p2::exec(src) },
            }
        },
        19 => {
            match part {
                DayPart::One => { d19::p1::exec(src) },
                DayPart::Two => { unimplemented!() },
            }
        },
        _ => {panic!("Invalid day")}
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}