// Solution: Given a positive y initial velocity,
// the vertical height of the probe will always reach 0 with y-velocity = -(y-initial)
// Thus, assuming a target area with y < 0, intersection with the target area
// requires only checking (0 - y-initial+1, 0 - (y-initial+1) - (y-initial+2), ...)
// until a point intersects or skips the target area

// Given that the probe will always be at y=0 and will next proceed to a height of (0-y-initial+1)
// it is trivial to calculate the maximum inital y as:
//     the summation of the absolute value of the lower height bound of the target area

// the summation of this max initial y-velocity is the result

// Note: x is irrelevant since it is independent from y
pub fn exec(src: &str, print: bool) {

    let line = src.lines().next().unwrap();

    let stripped = line.strip_prefix("target area: ").unwrap();
    let y_define = stripped.split(", ").last().unwrap();

    let mut y_vals = y_define.strip_prefix("y=").unwrap().split("..");

    let y_range: (i64, i64) = ( y_vals.next().unwrap().parse::<i64>().unwrap(), y_vals.next().unwrap().parse::<i64>().unwrap() );

    if print { println!("result: {}", (0..y_range.0.abs()).sum::<i64>()) }
}