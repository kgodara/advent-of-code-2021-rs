

pub fn exec (src: String) {
    let mut lines = src.lines();

    let p1_init_pos: u16 = str::parse(lines.next().unwrap().split(" starting position: ").last().unwrap()).unwrap();
    let p2_init_pos: u16 = str::parse(lines.next().unwrap().split(" starting position: ").last().unwrap()).unwrap();

    // Approach:
    //     Use 10x10x21x21x2 (21 since 21 discrete values (0,20)) array coordinates to store all universe info
    //         value of cells will be # of scenarios in given state
    //         Iterate over all cells (unique scenarios), execute roll, incr values at new scenarios,
    //             until no unfinished games left (all cells = 0)

    let mut all_roll_sums: Vec<usize> = vec![];

    for x in 1..4 {
        for y in 1..4 {
            for z in 1..4 {
                all_roll_sums.push(x+y+z);
            }
        }
    }

    // there are 21 distinct scores (0, 21), indexes are directly equal to score value
    let mut scenarios: Vec<Vec<Vec<Vec<Vec<u64>>>>> = vec![vec![vec![vec![vec![0; 2]; 21]; 21]; 10]; 10];

    // starting scenario, p1 turn first
    scenarios[(p1_init_pos-1) as usize][(p2_init_pos-1) as usize][0][0][0] = 1;

    let mut scenarios_remain: bool;

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    loop {

        scenarios_remain = false;
        // iterate over all possible scenarios, if scenarios[..][..][..][..] > 0, these scenarios exist
        for p1_pos in 0..10 {
            for p2_pos in 0..10 {
                for p1_score in 0..21 {
                    for p2_score in 0..21 {

                        // Note: die is still rolled 3x, each roll creates 3 new scenarios
                        // so each roll creates 27 scenarios

                        // p1's turn
                        if scenarios[p1_pos][p2_pos][p1_score][p2_score][0] > 0 {
                            // exec roll
                            scenarios_remain = true;
                            let num_cur_scenario: u64 = scenarios[p1_pos][p2_pos][p1_score][p2_score][0];

                            // For each of all 27 sums:
                            // if player has won incr by num universes, else increment each new universe with num_cur_scenario
                            for roll_sum in all_roll_sums.iter() {
                                // p1_pos [1-10]
                                let new_p1_pos: usize = ((p1_pos + roll_sum) % 10) + 1;

                                if (p1_score + new_p1_pos) >= 21 {
                                    p1_wins += num_cur_scenario;
                                } else {
                                    scenarios[new_p1_pos-1][p2_pos][p1_score+new_p1_pos][p2_score][1] += num_cur_scenario;
                                }
                            }
                            // these scenarios have now all progressed
                            scenarios[p1_pos][p2_pos][p1_score][p2_score][0] = 0;
                        }

                        // p2's turn
                        if scenarios[p1_pos][p2_pos][p1_score][p2_score][1] > 0 {
                            // exec roll
                            scenarios_remain = true;
                            let num_cur_scenario: u64 = scenarios[p1_pos][p2_pos][p1_score][p2_score][1];

                            // For each of all 27 sums:
                            // if player has won incr by num universes, else increment each new universe with num_cur_scenario
                            for roll_sum in all_roll_sums.iter() {
                                // p2_pos [1-10]
                                let new_p2_pos: usize = ((p2_pos + roll_sum) % 10)+1;

                                if (p2_score + new_p2_pos) >= 21 {
                                    p2_wins += num_cur_scenario;
                                } else {
                                    scenarios[p1_pos][new_p2_pos-1][p1_score][p2_score+new_p2_pos][0] += num_cur_scenario;
                                }
                            }
                            // these scenarios have now all progressed
                            scenarios[p1_pos][p2_pos][p1_score][p2_score][1] = 0;
                        }
                    }
                }
            }
        }

        if !scenarios_remain {
            break;
        }
    }

    println!("result: {}", std::cmp::max(p1_wins, p2_wins));

}