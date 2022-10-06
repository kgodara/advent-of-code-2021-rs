use std::collections::{BTreeSet, VecDeque};

use std::collections::HashSet;
use fnv::FnvHashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

fn rotations_fast_vec(scanner: &[Point], k: &mut Vec<Vec<Point>>) {
    for _ in 0..24 {
        k.push(Vec::with_capacity(27));
    }

    for s in scanner.iter() {
        k[0].push(Point { x: s.x, y: s.y, z: s.z });
        k[1].push(Point { x: s.y, y: s.z, z: s.x });
        k[2].push(Point { x: s.z, y: s.y, z: -s.x });
        k[3].push(Point { x: s.x, y: -s.z, z: s.y });
        k[4].push(Point { x: s.y, y: -s.x, z: s.z });
        k[5].push(Point { x: s.z, y: s.x, z: s.y });
        k[6].push(Point { x: s.x, y: -s.y, z: -s.z });
        k[7].push(Point { x: s.y, y: -s.z, z: -s.x });
        k[8].push(Point { x: s.z, y: -s.y, z: s.x });
        k[9].push(Point { x: s.x, y: s.z, z: -s.y });
        k[10].push(Point { x: s.y, y: s.x, z: -s.z });
        k[11].push(Point { x: s.z, y: -s.x, z: -s.y });
        k[12].push(Point { x: -s.x, y: s.y, z: -s.z });
        k[13].push(Point { x: -s.y, y: s.z, z: -s.x });
        k[14].push(Point { x: -s.z, y: s.y, z: s.x });
        k[15].push(Point { x: -s.x, y: -s.z, z: -s.y });
        k[16].push(Point { x: -s.y, y: -s.x, z: -s.z });
        k[17].push(Point { x: -s.z, y: s.x, z: -s.y });
        k[18].push(Point { x: -s.x, y: -s.y, z: s.z });
        k[19].push(Point { x: -s.y, y: -s.z, z: s.x });
        k[20].push(Point { x: -s.z, y: -s.y, z: -s.x });
        k[21].push(Point { x: -s.x, y: s.z, z: s.y });
        k[22].push(Point { x: -s.y, y: s.x, z: s.z });
        k[23].push(Point { x: -s.z, y: -s.x, z: s.y });
    }
}


// all 24 possible rotations/permutations of the set of points for a given scanner
fn rotations(mut s: Vec<Point>, k: &mut Vec<Vec<Point>>) {
    for _ in 0..4 {
        for _ in 0..4 {
            k.push(s.clone());
            s = s.into_iter()
                .map(|p| {
                    Point { x: p.z, y: p.y, z: -p.x }
                })
                .collect();
        }
        k.push(
            s.iter()
                .cloned()
                .map(|p| {
                    Point { x: p.y, y: -p.x, z: p.z }
                })
                .collect()
        );

        k.push(
            s.iter()
                .cloned()
                .map(|p| {
                    Point { x: -p.y, y: p.x, z: p.z }
                })
                .collect()
        );

        s = s.into_iter()
            .map(|p| {
                Point { x: p.x, y: p.z, z: -p.y }
            })
            .collect();
    }
}

fn calc_intersect<'a>(
    s1: &BTreeSet<Point>, 
    s2: &Vec<Point>,
    k: &'a mut Vec<Vec<Point>>)
    -> Option<(impl Iterator<Item = Point> + 'a, Point)> {

    // let now = std::time::Instant::now();
    k.drain(..);


    let mut s1_set: FnvHashSet<Point> = FnvHashSet::default();
    for point in s1.iter() {
        s1_set.insert(*point);
    }
    rotations_fast_vec(s2, k);

    // println!("calc_intersect() init Elapsed: {:.2?}", now.elapsed());

    // iter over all 24 rotations of s2
    for ss2 in k.iter() {
        // iter over every point in s1
        for a in s1.iter() {
            // iter over all rotated s2 points
            for b in ss2.iter() {


                let off = *b - *a;

                let mut num_common: u16 = 0;

                for b in ss2.iter() {
                    if s1_set.contains(&(*b - off)) {
                        num_common += 1;
                    }
                    if num_common >= 12 {
                        break;
                    }
                }

                if num_common >= 12 {
                    return Some((ss2
                        .iter()
                        .map(move |b| {
                            *b - off
                        }), off));
                }
            }
        }
    }

    None
}

pub fn exec(src: &str, print: bool) {

    let mut scanners: Vec<Vec<Point>> = vec![vec![]];


    for line in src.lines() {
        if line.trim().is_empty() {
            scanners.push(vec![]);
        }
        else if !line.contains("scanner ") {
            let mut nums_str = line.split(',');
            scanners.last_mut().unwrap().push(Point {
                x: nums_str.next().unwrap().parse::<i32>().unwrap(),
                y: nums_str.next().unwrap().parse::<i32>().unwrap(),
                z: nums_str.next().unwrap().parse::<i32>().unwrap()
            });
        }
    }


    let mut t: BTreeSet<Point> = BTreeSet::from_iter(scanners[0].iter().cloned());

    let mut o: Vec<Point> = vec![ Point{x:0, y:0, z:0} ];

    let mut q: VecDeque<Vec<Point>> = VecDeque::from_iter(scanners.into_iter().skip(1));

    // total rotations: 24
    // scanner max-point-num: 27
    // let mut k: [[Point; 27]; 24] = [[Point::default(); 27]; 24];
    let mut k: Vec<Vec<Point>> = vec![];

    while !q.is_empty() {


        let k = calc_intersect(&t, &q[0], &mut k);

        if let Some(k) = k {
            t = t.union(&BTreeSet::from_iter(k.0)).cloned().collect();
            o.push(k.1);
            q.pop_front();
        }
        else {
            let temp = q.pop_front().unwrap();
            q.push_back(temp);
        }
    }

    let mut md = 0;

    for a in o.iter() {
        for b in o.iter() {
            md = std::cmp::max(md, (a.x -b.x).abs() + (a.y-b.y).abs() + (a.z-b.z).abs());// sum(abs(x - y) for x, y in zip(a, b)));
        }
    }

    if print { println!("result: {}", md) }
}