use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use aoc21::d1;
use aoc21::d2;
use aoc21::d3;
use aoc21::d4;
use aoc21::d5;
use aoc21::d6;
use aoc21::d7;
use aoc21::d8;
use aoc21::d9;
use aoc21::d10;
use aoc21::d11;
use aoc21::d12;
use aoc21::d13;
use aoc21::d14;
use aoc21::d15;
use aoc21::d16;
use aoc21::d17;
use aoc21::d18;
use aoc21::d19;
use aoc21::d20;
use aoc21::d21;
use aoc21::d22;
use aoc21::d23;
use aoc21::d24;
use aoc21::d25;

#[allow(clippy::unit_arg)]
fn separate(c: &mut Criterion) {

    let in_list = vec![
        include_str!("../src/input/d1.txt"),
        include_str!("../src/input/d2.txt"),
        include_str!("../src/input/d3.txt"),
        include_str!("../src/input/d4.txt"),
        include_str!("../src/input/d5.txt"),
        include_str!("../src/input/d6.txt"),
        include_str!("../src/input/d7.txt"),
        include_str!("../src/input/d8.txt"),
        include_str!("../src/input/d9.txt"),
        include_str!("../src/input/d10.txt"),
        include_str!("../src/input/d11.txt"),
        include_str!("../src/input/d12.txt"),
        include_str!("../src/input/d13.txt"),
        include_str!("../src/input/d14.txt"),
        include_str!("../src/input/d15.txt"),
        include_str!("../src/input/d16.txt"),
        include_str!("../src/input/d17.txt"),
        include_str!("../src/input/d18.txt"),
        include_str!("../src/input/d19.txt"),
        include_str!("../src/input/d20.txt"),
        include_str!("../src/input/d21.txt"),
        include_str!("../src/input/d22.txt"),
        include_str!("../src/input/d23.txt"),
        include_str!("../src/input/d24.txt"),
        include_str!("../src/input/d25.txt"),
    ];

    {
        let mut group = c.benchmark_group("d1");
        group.bench_with_input("p1", &in_list[0], |b, &s| b.iter(|| black_box(d1::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[0], |b, &s| b.iter(|| black_box(d1::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d2");
        group.bench_with_input("p1", &in_list[1], |b, &s| b.iter(|| black_box(d2::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[1], |b, &s| b.iter(|| black_box(d2::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d3");
        group.bench_with_input("p1", &in_list[2], |b, &s| b.iter(|| black_box(d3::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[2], |b, &s| b.iter(|| black_box(d3::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d4");
        group.bench_with_input("p1", &in_list[3], |b, &s| b.iter(|| black_box(d4::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[3], |b, &s| b.iter(|| black_box(d4::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d5");
        group.bench_with_input("p1", &in_list[4], |b, &s| b.iter(|| black_box(d5::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[4], |b, &s| b.iter(|| black_box(d5::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d6");
        group.bench_with_input("p1", &in_list[5], |b, &s| b.iter(|| black_box(d6::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[5], |b, &s| b.iter(|| black_box(d6::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d7");
        group.bench_with_input("p1", &in_list[6], |b, &s| b.iter(|| black_box(d7::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[6], |b, &s| b.iter(|| black_box(d7::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d8");
        group.bench_with_input("p1", &in_list[7], |b, &s| b.iter(|| black_box(d8::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[7], |b, &s| b.iter(|| black_box(d8::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d9");
        group.bench_with_input("p1", &in_list[8], |b, &s| b.iter(|| black_box(d9::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[8], |b, &s| b.iter(|| black_box(d9::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d10");
        group.bench_with_input("p1", &in_list[9], |b, &s| b.iter(|| black_box(d10::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[9], |b, &s| b.iter(|| black_box(d10::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d11");
        group.bench_with_input("p1", &in_list[10], |b, &s| b.iter(|| black_box(d11::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[10], |b, &s| b.iter(|| black_box(d11::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d12");
        group.bench_with_input("p1", &in_list[11], |b, &s| b.iter(|| black_box(d12::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[11], |b, &s| b.iter(|| black_box(d12::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d13");
        group.bench_with_input("p1", &in_list[12], |b, &s| b.iter(|| black_box(d13::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[12], |b, &s| b.iter(|| black_box(d13::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d14");
        group.bench_with_input("p1", &in_list[13], |b, &s| b.iter(|| black_box(d14::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[13], |b, &s| b.iter(|| black_box(d14::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d15");
        group.bench_with_input("p1", &in_list[14], |b, &s| b.iter(|| black_box(d15::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[14], |b, &s| b.iter(|| black_box(d15::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d16");
        group.bench_with_input("p1", &in_list[15], |b, &s| b.iter(|| black_box(d16::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[15], |b, &s| b.iter(|| black_box(d16::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d17");
        group.bench_with_input("p1", &in_list[16], |b, &s| b.iter(|| black_box(d17::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[16], |b, &s| b.iter(|| black_box(d17::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d18");
        group.bench_with_input("p1", &in_list[17], |b, &s| b.iter(|| black_box(d18::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[17], |b, &s| b.iter(|| black_box(d18::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d19");
        group.sample_size(10);
        group.bench_with_input("p1", &in_list[18], |b, &s| b.iter(|| black_box(d19::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[18], |b, &s| b.iter(|| black_box(d19::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d20");
        group.bench_with_input("p1", &in_list[19], |b, &s| b.iter(|| black_box(d20::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[19], |b, &s| b.iter(|| black_box(d20::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d21");
        group.bench_with_input("p1", &in_list[20], |b, &s| b.iter(|| black_box(d21::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[20], |b, &s| b.iter(|| black_box(d21::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d22");
        group.bench_with_input("p1", &in_list[21], |b, &s| b.iter(|| black_box(d22::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[21], |b, &s| b.iter(|| black_box(d22::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d23");
        group.bench_with_input("p1", &in_list[22], |b, &s| b.iter(|| black_box(d23::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[22], |b, &s| b.iter(|| black_box(d23::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d24");
        group.bench_with_input("p1", &in_list[23], |b, &s| b.iter(|| black_box(d24::p1::exec(s, false))));
        group.bench_with_input("p2", &in_list[23], |b, &s| b.iter(|| black_box(d24::p2::exec(s, false))));
    }
    {
        let mut group = c.benchmark_group("d25");
        group.bench_with_input("p1", &in_list[24], |b, &s| b.iter(|| black_box(d25::p1::exec(s, false))));
    }
}



fn all_at_once(c: &mut Criterion) {

    let in_list = vec![
        include_str!("../src/input/d1.txt"),
        include_str!("../src/input/d2.txt"),
        include_str!("../src/input/d3.txt"),
        include_str!("../src/input/d4.txt"),
        include_str!("../src/input/d5.txt"),
        include_str!("../src/input/d6.txt"),
        include_str!("../src/input/d7.txt"),
        include_str!("../src/input/d8.txt"),
        include_str!("../src/input/d9.txt"),
        include_str!("../src/input/d10.txt"),
        include_str!("../src/input/d11.txt"),
        include_str!("../src/input/d12.txt"),
        include_str!("../src/input/d13.txt"),
        include_str!("../src/input/d14.txt"),
        include_str!("../src/input/d15.txt"),
        include_str!("../src/input/d16.txt"),
        include_str!("../src/input/d17.txt"),
        include_str!("../src/input/d18.txt"),
        include_str!("../src/input/d19.txt"),
        include_str!("../src/input/d20.txt"),
        include_str!("../src/input/d21.txt"),
        include_str!("../src/input/d22.txt"),
        include_str!("../src/input/d23.txt"),
        include_str!("../src/input/d24.txt"),
        include_str!("../src/input/d25.txt"),
    ];

    #[allow(clippy::unit_arg)]
    c.bench_with_input(BenchmarkId::new("all", ""), &in_list, |b, s| {
        b.iter(|| {
            black_box({
                d1::p1::exec(s[0], false);
                d1::p2::exec(s[0], false);

                d2::p1::exec(s[1], false);
                d2::p2::exec(s[1], false);

                d3::p1::exec(s[2], false);
                d3::p2::exec(s[2], false);

                d4::p1::exec(s[3], false);
                d4::p2::exec(s[3], false);

                d5::p1::exec(s[4], false);
                d5::p2::exec(s[4], false);

                d6::p1::exec(s[5], false);
                d6::p2::exec(s[5], false);

                d7::p1::exec(s[6], false);
                d7::p2::exec(s[6], false);

                d8::p1::exec(s[7], false);
                d8::p2::exec(s[7], false);

                d9::p1::exec(s[8], false);
                d9::p2::exec(s[8], false);

                d10::p1::exec(s[9], false);
                d10::p2::exec(s[9], false);

                d11::p1::exec(s[10], false);
                d11::p2::exec(s[10], false);

                d12::p1::exec(s[11], false);
                d12::p2::exec(s[11], false);

                d13::p1::exec(s[12], false);
                d13::p2::exec(s[12], false);

                d14::p1::exec(s[13], false);
                d14::p2::exec(s[13], false);

                d15::p1::exec(s[14], false);
                d15::p2::exec(s[14], false);

                d16::p1::exec(s[15], false);
                d16::p2::exec(s[15], false);

                d17::p1::exec(s[16], false);
                d17::p2::exec(s[16], false);

                d18::p1::exec(s[17], false);
                d18::p2::exec(s[17], false);

                d19::p1::exec(s[18], false);
                d19::p2::exec(s[18], false);

                d20::p1::exec(s[19], false);
                d20::p2::exec(s[19], false);

                d21::p1::exec(s[20], false);
                d21::p2::exec(s[20], false);

                d22::p1::exec(s[21], false);
                d22::p2::exec(s[21], false);

                d23::p1::exec(s[22], false);
                d23::p2::exec(s[22], false);

                d24::p1::exec(s[23], false);
                d24::p2::exec(s[23], false);

                d25::p1::exec(s[24], false);

            })
        })
    });
}

criterion_group!(benches, separate, all_at_once);
criterion_main!(benches);
