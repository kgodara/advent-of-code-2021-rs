#![allow(dead_code)]
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

mod util;

// #[]
fn main() {

    use std::time::Instant;
    let now = Instant::now();

    /* DAY 1
        d1::p1::exec();
        d1::p2::exec();
    */

    /* DAY 2
        d2::p1::exec();
        d2::p2::exec();
    */

    /* DAY 3
        d3::p1::exec();
        d3::p2::exec();
    */

    /* DAY 4
        d4::p1::exec();
        d4::p2::exec();
    */

    /* DAY 5
        d5::p1::exec();
        d5::p2::exec();
    */

    /* DAY 6
        d6::p1::exec();
        d6::p2::exec();
    */

    d7::p1::exec();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}