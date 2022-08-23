use std::cmp::{max, min, Ordering};
/*
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
*/

// RULES:

// 1. If any unit has a direct path to its room (empty or has matching unit), that move is optimal.
// 2. Each unit only has MAX 2 moves: into hallway OR into dest room (from room or hallway)

// Seems like a pretty easy case for BFS/DFS with a stack and enumerate all win conditions
// max 44 possible initial moves, ~ max 40 possible after, etc.

// Other idea: Use heuristic of minimal moves to satisfy room in order to prioritize ordering


// Execute the first possible direct move into target room
// return true if move executed, false if none found
fn attempt_direct_move(hallway: &mut Vec<u8>, rooms: &mut Vec<Vec<u8>>, cost: &mut u64) -> bool {

    // list of room positions in hall
    let room_hall_idx_list: Vec<u8> = vec![2, 4, 6, 8];

    // (end_room_idx, end_room_depth_idx)
    let candidate_end_rooms: Vec<(usize, usize)> = rooms.iter()
        .enumerate()
        .filter_map(|(room_idx, room)| {

            // Assumption: never occupied spots above an empty spot

            // bottom spot (4th) empty 
            if room[3] == 0 {
                Some((room_idx, 3))
            }

            // 3d spot empty & spots below are valid
            else if room[2] == 0 && room[3] == (room_idx+1) as u8 {
                Some((room_idx, 2))
            } 

            // 2nd spot empty & spots below are valid
            else if room[1] == 0 && !room[2..4].iter().any(|occupant| *occupant != ((room_idx+1) as u8)) {
                Some((room_idx, 1))
            }

            // 1st spot empty & spots below are valid
            else if room[0] == 0 && !room[1..4].iter().any(|occupant| *occupant != ((room_idx+1) as u8)) {
                Some((room_idx, 0))
            }

            else {
                None
            }
        })
        .collect();


    // for each room which can be moved into
    // check for an amphipod which can directly move from another room

    // end_room: (room_idx, can_move_bottom_spot)
    for end_room in candidate_end_rooms.iter() {

        let (end_room_idx, end_room_depth_idx): (usize, usize) = (end_room.0, end_room.1);
        let end_room_occupant: u8 = (end_room.0+1) as u8;

        for (start_room_idx, start_room) in rooms.iter_mut().enumerate() {

            let mut valid_start_found: bool = false;
            let mut start_depth_idx: u8 = 0;

            // start_room same as end_room, continue;
            if start_room_idx == end_room_idx { continue; }


            // start_room top occupant matches end_room desired occupant
            if start_room[0] == end_room_occupant {
                valid_start_found = true;
                start_depth_idx = 0;
            }

            // start_room bottom occupant matches end_room desired occupant
            // && start_room top occupant is empty
            else if start_room[1] == end_room_occupant && start_room[0] == 0 {
                valid_start_found = true;
                start_depth_idx = 1;
            }

            // start_room 3d occupant matches end_room desired occupant
            // && start_room above occupants empty
            else if start_room[2] == end_room_occupant &&
            start_room[0] == 0 && start_room[1] == 0 {
                valid_start_found = true;
                start_depth_idx = 2;
            }

            // start_room 4th occupant matches end_room desired occupant
            // && start_room above occupants empty
            else if start_room[3] == end_room_occupant &&
            start_room[0] == 0 && start_room[1] == 0 && start_room[2] == 0 {
                valid_start_found = true;
                start_depth_idx = 3;
            }

            // if occupant matching end_room found:
            if valid_start_found {

                // room_hall_idx_list[start_room_idx]..room_hall_idx_list[end_room_idx]
                let start_room_hall_idx = room_hall_idx_list[start_room_idx];
                let end_room_hall_idx = room_hall_idx_list[end_room_idx];

                // determine if unobstructed path exists start_room -> end_room
                let is_path_blocked: bool = 
                    hallway[(min(start_room_hall_idx, end_room_hall_idx) as usize)..((max(start_room_hall_idx, end_room_hall_idx)+1) as usize)]
                    .iter()
                    .any(|path_pos| *path_pos > 0);

                if !is_path_blocked {

                    start_room[start_depth_idx as usize] = 0;
                    rooms[end_room_idx][end_room_depth_idx] = end_room_occupant;

                    // calculate cost
                    *cost += calc_move_cost(
                        end_room_occupant,
                        start_room_hall_idx, end_room_hall_idx,
                        start_depth_idx+1, (end_room_depth_idx+1) as u8
                    );

                    return true;
                }                
            }
        }
    }

    // for each room which can be moved into,
    // check for an amphipod which can directly move from a hallway
    // end_room: (room_idx, can_move_bottom_spot)
    for end_room in candidate_end_rooms.iter() {
        let (end_room_idx, end_room_depth_idx) = (end_room.0, end_room.1);
        let end_room_occupant: u8 = (end_room.0+1) as u8;        
        let end_room_hall_idx = room_hall_idx_list[end_room.0];

        // check all hallway positions
        for (hall_pos, hall_occupant) in hallway.iter().enumerate() {

            // check hall occupied at position and matches end_room desired occupant
            if *hall_occupant == ((end_room_idx+1) as u8) {

                // determine if unobstructed path exists hall_pos -> end_room
                // NOTE: hall_pos + 1, when hall_pos is lower since hall_pos is always occupied

                let (lower_bound, upper_bound) = match hall_pos.cmp(&(end_room_hall_idx as usize)) {
                    Ordering::Less => { (hall_pos+1, (end_room_hall_idx+1) as usize)  },
                    Ordering::Greater => { (end_room_hall_idx as usize, hall_pos) },
                    Ordering::Equal => { (hall_pos, end_room_hall_idx as usize) }
                };

                let is_path_blocked: bool =
                    hallway[lower_bound..upper_bound]
                    .iter()
                    .any(|path_pos| *path_pos > 0);

                if !is_path_blocked {
                    hallway[hall_pos] = 0;
                    rooms[end_room_idx][end_room_depth_idx] = end_room_occupant;

                    // calculate cost
                    *cost += calc_move_cost(
                        end_room_occupant,
                        hall_pos as u8, end_room_hall_idx,
                        0, (end_room_depth_idx+1) as u8
                    );

                    return true;
                }
            }
        }
    }

    false
}

fn calc_move_cost(occupant: u8, hall_start_idx: u8, hall_stop_idx: u8, depth_start: u8, depth_end: u8) -> u64 {

    // amphipod-cost lookup
    let cost_lookup: Vec<u64> = vec![1, 10, 100, 1000];
   
    let mut new_cost: u64 = (max(hall_start_idx, hall_stop_idx) - min(hall_start_idx, hall_stop_idx)) as u64;
    new_cost += (depth_start + depth_end) as u64;
    new_cost *= cost_lookup[(occupant-1) as usize];
    new_cost
}

fn print_burrow(hallway: &[u8], rooms: &[Vec<u8>], cost: &u64) {

    println!("COST: {}", cost);

    let ch_from_val = |val| {
        match val {
            0 => {'.'},
            1 => {'A'},
            2 => {'B'},
            3 => {'C'},
            4 => {'D'},
            _ => {panic!("Invalid hall value")}
        }
    };

    for hall_pos in hallway.iter() {
        print!("{}", ch_from_val(*hall_pos))
    }
    println!();

    print!(" #");
    for room in rooms.iter() {
        print!("{}#", ch_from_val(room[0]));
    }
    println!();

    print!(" #");
    for room in rooms.iter() {
        print!("{}#", ch_from_val(room[1]));
    }
    println!();

    print!(" #");
    for room in rooms.iter() {
        print!("{}#", ch_from_val(room[2]));
    }
    println!();

    print!(" #");
    for room in rooms.iter() {
        print!("{}#", ch_from_val(room[3]));
    }
    println!("\n");
}


pub fn exec(src: String) {
    let mut lines = src.lines();
    lines.next();
    // hallway[x] = room_index of occupant (0=vacant)
    let hallway: Vec<u8> = vec![0; lines.next().unwrap().trim().trim_matches('#').chars().count()];
    let mut rooms: Vec<Vec<u8>> = vec![];


    let val_from_occupant = |occupant| {match occupant {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        _ => panic!("invalid amphipod!")
    }};

    // top-level room occupants
    for room_occupant in lines.next().unwrap().trim().replace('#',"").chars() {
        rooms.push(vec![]);
        let room_len = rooms.len();
        rooms[room_len-1].push(val_from_occupant(room_occupant));
    }

    // Hard-coded second and third level room occupants, so same input can be used for p1 & p2

    /*
        D#C#B#A
    */
    rooms[0].push(4);
    rooms[1].push(3);
    rooms[2].push(2);
    rooms[3].push(1);


    /*
        #D#B#A#C#
    */
    rooms[0].push(4);
    rooms[1].push(2);
    rooms[2].push(1);
    rooms[3].push(3);


    // bottom-level room occupants
    for (char_idx, room_occupant) in lines.next().unwrap().trim().replace('#',"").chars().enumerate() {
        rooms[char_idx].push(val_from_occupant(room_occupant));
    }

    // (hallway, rooms, cost)
    let mut state_stack: Vec<(Vec<u8>, Vec<Vec<u8>>, u64)> = vec![(hallway, rooms, 0)];

    // list of room positions in hall
    let room_hall_idx_list: Vec<u8> = vec![2, 4, 6, 8];

    let mut min_cost: u64 = u64::MAX;

    while !state_stack.is_empty() {

        let cur_state = state_stack.pop().unwrap();

        let (mut cur_hallway, mut cur_rooms, mut cur_cost) = (cur_state.0, cur_state.1, cur_state.2);


        // check if state is complete
        let is_not_done: bool = cur_rooms.iter().enumerate().any(|(room_idx, room)| {
            room.iter().any(|occupant| *occupant != ((room_idx+1) as u8))
        });


        if !is_not_done {
            min_cost = min(min_cost, cur_cost);
            continue;
        }


        // check for direct move to final location
        // if found, exec, push new state, and stop enumeration
        if attempt_direct_move(&mut cur_hallway, &mut cur_rooms, &mut cur_cost) {
            state_stack.push((cur_hallway, cur_rooms, cur_cost));
            continue;
        }

        // (room_idx, depth_idx)
        let can_move_to_hall_list: Vec<(usize, usize)> = cur_rooms.iter()
            .enumerate()
            .filter_map(|(room_idx, room)| {

                let room_intended_occ: u8 = (room_idx + 1) as u8;

                // move top: top-occupant_not_empty and (invalid or below_has_invalid)
                if room[0] != 0 &&
                (room[0] != room_intended_occ || room[1..].iter().any(|occ| *occ != room_intended_occ)) 
                {
                    Some((room_idx, 0))
                }

                // move second: above_is_empty and second_not_empty and (invalid or below_has_invalid)
                else if !room[..1].iter().any(|x| *x != 0) &&
                room[1] != 0 &&
                (room[1] != room_intended_occ || room[2..].iter().any(|occ| *occ != room_intended_occ)) {
                    Some((room_idx, 1))
                }

                // move third: above_is_empty and third_not_empty and (invalid or below_has_invalid)
                else if !room[..2].iter().any(|x| *x != 0) &&
                room[2] != 0 &&
                (room[2] != room_intended_occ || room[3..].iter().any(|occ| *occ != room_intended_occ)) {
                    Some((room_idx, 2))
                }

                // move bottom: above_is_empty and bottom_not_empty and invalid
                else if !room[..3].iter().any(|x| *x != 0) &&
                room[3] != 0 &&
                room[3] != room_intended_occ
                {
                    Some((room_idx, 3))
                }

                // nothing can move out of this room
                else {
                    None
                }
            })
            .collect();

        // Check for move from invalid room occupant into unoccupied hall space
        // Make sure not to allow moving past other amphipods
        // Do not allow moving into hall space over the entrance to a room

        // (room_idx, depth)
        for occupant in can_move_to_hall_list.iter() {
            let (occupant_room_idx, occupant_depth_idx): (usize, usize) = (occupant.0, occupant.1);

            // iter over hallway positions moving left & right

            let mut left_idx: i8 = (room_hall_idx_list[occupant_room_idx]-1) as i8;
            let mut right_idx: i8 = (room_hall_idx_list[occupant_room_idx]+1) as i8;

            let mut valid_spaces: Vec<u8> = vec![];

            while left_idx >= 0 || right_idx < (cur_hallway.len() as i8) {

                // range check and not above room check
                if left_idx >= 0 && (left_idx % 2 != 0 || left_idx == 0) {
                    // if unoccupied
                    if cur_hallway[left_idx as usize] == 0 {
                        valid_spaces.push(left_idx as u8);
                    }
                    // obstruction found, stop considering further left positions
                    else {
                        left_idx = -1;
                    }
                }

                // range check and not above room check
                if right_idx < (cur_hallway.len() as i8) && (right_idx % 2 != 0 || right_idx > 8) {
                    // if unoccupied
                    if cur_hallway[right_idx as usize] == 0 {
                        valid_spaces.push(right_idx as u8);
                    }
                    // obstruction found, stop considering further left positions
                        else {
                        // println!("right_idx obstruction found: {}", right_idx);
                        right_idx = cur_hallway.len() as i8;
                    }
                }
                left_idx -= 1;
                right_idx += 1;
            }

            // generate a new state for each space in valid_spaces
            // occupant: (room_idx, depth)
            // space: hall_pos
            for space in valid_spaces.iter() {

                let mut new_rooms = cur_rooms.clone();
                new_rooms[occupant_room_idx][occupant_depth_idx] = 0;

                let mut new_hallway = cur_hallway.clone();

                // set to value of occupant moving out of room
                let unit_moving = cur_rooms[occupant_room_idx][occupant_depth_idx];
                new_hallway[*space as usize] = unit_moving;


                // calculate cost
                let new_cost: u64 = cur_cost + calc_move_cost(
                    unit_moving,
                    room_hall_idx_list[occupant_room_idx] as u8, *space,
                    (occupant_depth_idx+1) as u8, 0
                );

                state_stack.push((new_hallway, new_rooms, new_cost));
            }
        }
    }

    println!("result: {}", min_cost);
}