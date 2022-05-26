// use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn calc_point_dist(pt_one: &Point, pt_two: &Point) -> u64 {
    f64::sqrt(((pt_one.x - pt_two.x).pow(2) + (pt_one.y - pt_two.y).pow(2) + (pt_one.z - pt_two.z).pow(2)) as f64) as u64
}

fn test_membership(scanner_points: &[Point], _candidate_points: &Vec<Point>) {
    // Iterate through all 24 orientations and see if any result in 12 points matching scanner_points

    // Question: Can we map an individual point to another individual point?
    //     No, since invidivual points can have completely arbitrary values based on scanner position

    // Proposal: Test all candidate distance pairs against all set distance pairs, add any new potential beacons to list of common beacons
    // if len(common_beacons) >= COMMON_THRESHOLD, consider scanners overlapping
    //     Question: Is 6-12 of the same distances between beacons from two scanners enough to say that beacons are
    //     Note: Use a hashmap with freq to store distance pairs from non-candidate scanner,
    //         decrement on removal to prevent the same pair from being matched on multiple times
    // TEST: Add a hashset for seen distances 

    // Proposal: Test all 24 orientations of scanner, calculate possible position of 'scanner' against 'candidate' by:
    //     comparing each 'candidate' point against all 'scanner' points 

    // Generate HashMap of all distances in scanner_points
    // dist -> freq

    let mut dists_to_match: HashMap<u64, u8> = HashMap::new();
    for (idx_one, point_one) in scanner_points.iter().enumerate() {
        for (idx_two, point_two) in scanner_points.iter().enumerate() {
            if idx_one == idx_two {
                continue;
            }

            let temp = calc_point_dist(point_one, point_two);
            // println!("{}", temp);

            if dists_to_match.contains_key(&temp) {
                *dists_to_match.get_mut(&temp).unwrap() += 1;
            } else {
                dists_to_match.insert(temp, 1);
            }
        }
    }

    println!("dists_to_match: {:?}", dists_to_match);



    // for candidate_beacon

}

pub fn exec(src: String) {

    let mut scanner_points: Vec<Vec<Point>> = vec![vec![]];

    for line in src.lines() {
        if line.trim().is_empty() {
            scanner_points.push(vec![]);
        }
        else if !line.contains("scanner ") {
            let mut nums_str = line.split(',');
            scanner_points.last_mut().unwrap().push(Point {
                x: nums_str.next().unwrap().parse::<i32>().unwrap(),
                y: nums_str.next().unwrap().parse::<i32>().unwrap(),
                z: nums_str.next().unwrap().parse::<i32>().unwrap()
            });
        }
    }
    // println!("scanner_points: {:?}", scanner_points);

    for scanner_one in scanner_points.iter() {
        test_membership(scanner_one, &scanner_points[0]);
    }

}