#![allow(dead_code)]
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

mod util;

// #[]
fn main() {

    use std::time::Instant;
    let now = Instant::now();

    /*
        d1::p1::exec();
        d1::p2::exec();

        d2::p1::exec();
        d2::p2::exec();

        d3::p1::exec();
        d3::p2::exec();

        d4::p1::exec();
        d4::p2::exec();

        d5::p1::exec();
        d5::p2::exec();

        d6::p1::exec();
        d6::p2::exec();

        d6::p1::exec();
        d6::p2::exec();
    
        d7::p1::exec();
        d7::p2::exec();

        d8::p1::exec();
        d8::p2::exec();

        d9::p1::exec();
        d9::p2::exec();

        d10::p1::exec();
        d10::p2::exec();

        d11::p1::exec();
    */

    d11::p2::exec();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}