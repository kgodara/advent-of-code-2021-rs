# Advent of Code 2021-Rust

[Advent of Code 2021](https://adventofcode.com/2021/) Rust Solutions

A series of programming puzzles, solved using Rust. 

Written with the following goals in mind:

- Familiarizing myself with idiomatic Rust solutions
- Relative simplicity
- Reasonable performance (achieved, except for day 19)
- **Note**: part 1 & 2 for some days use different approaches, e.g. I used a better approach for part 2.


# Usage:

1. Clone this Repository.
2. `cd $CLONE_DIR/src/`
3. `cargo run --release -- -d 15 -p 1`

## Arguments:
    -d, --day <DAY>          Day to exec: (1, ..., 25)
    -h, --help               Print help information
    -p, --part <PART>        Day part to exec: (1, 2)
    -s, --suffix <SUFFIX>    Input file suffix: "input/d{DAY}{SUFFIX}.txt" (for additional inputs)
                             [default: ]
    -V, --version            Print version information

# Performance:

### Benchmarked Without Day 19: 371.14 ms
### Benchmarked With Day 19: 4.549 s

| Day | Part 1 | Part 2 | Cumulative |
| --- | ------ | ------ | ---------- |
| 1   | 27 µs   | 28 µs    | **0 ms** |
| 2   | 35 µs   | 36 µs    | **0 ms** |
| 3   | 77 µs   | 151 µs   | **0 ms** |
| 4   | 370 µs  | 719 µs   | **1 ms** |
| 5   | 588 µs  | 1.1 ms   | **3 ms** |
| 6   | 1.2 ms  | 470 µs   | **5 ms** |
| 7   | 25 µs   | 25 µs    | **5 ms** |
| 8   | 149 µs  | 805 µs   | **6 ms** |
| 9   | 756 µs  | 1.4 ms   | **8 ms** |
| 10  | 76 µs   | 77 µs    | **8 ms** |
| 11  | 121 µs  | 446 µs   | **8 ms** |
| 12  | 191 µs  | 5.4 ms   | **14 ms** |
| 13  | 300 µs  | 1.1 ms   | **15 ms** |
| 14  | 2.9 ms  | 118 µs   | **18 ms** |
| 15  | 2.2 ms  | 22.1 ms  | **43 ms** |
| 16  | 462 µs  | 574 µs   | **44 ms** |
| 17  | 140 ns  | 1.4 ms   | **45 ms** |
| 18  | 1.2 ms  | 17.2 ms  | **64 ms** |
| 19  | 1.98 s  | 2.23 s   | **4274 ms** |
| 20  | 1.9 ms  | 42.5 ms  | **4318 ms** |
| 21  | 832 ns  | 5.8 ms   | **4325 ms** |
| 22  | 10.4 ms | 10.4 ms  | **4346 ms** |
| 23  | 55.3 ms | 115.4 ms | **4517 ms** |
| 24  | 9 µs    | 9 µs     | **4517 ms** |
| 25  | 32.3 ms | NA       | **4549 ms** |
