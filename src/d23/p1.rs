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

// list of room positions (columns) in hall
const ROOM_HALL_IDX_LOOKUP: [u8; 4] = [2, 4, 6, 8];

// amphipod-cost lookup
const COST_LOOKUP: [u64; 4] = [1, 10, 100, 1000];

fn calc_move_cost(occupant: u8, hall_start_idx: u8, hall_stop_idx: u8, depth_start: u8, depth_end: u8) -> u64 {
   
    let mut new_cost: u64 = (max(hall_start_idx, hall_stop_idx) - min(hall_start_idx, hall_stop_idx)) as u64;
    new_cost += (depth_start + depth_end) as u64;
    
    // new_cost *= cost_lookup[(occupant-1) as usize];
    new_cost *= COST_LOOKUP[(occupant-1) as usize];

    new_cost
}


// Execute the first possible direct move into target room
// return true if move executed, false if none found
fn attempt_direct_move(hallway: &mut [u8; 11],
    rooms: &mut [[u8; 2]; 4],
    candidate_end_rooms: &mut [(usize, usize); 4],
    cost: &mut u64) -> bool {

    // procedure for identifying valid end room:
    //     iterate over room slots bottom-up:
    //         on empty slot:
    //             if all spots before were valid non-empty occupants, return: (room, depth)

    // (end_room_idx, end_room_depth_idx)
    let candidate_end_rooms_i = rooms.iter()
        .enumerate()
        .filter_map(|(room_idx, room)| {

            // find valid empty spot to move to
            // search bottom->top of room: never empty spots below occupied spots
            for (depth, room_occ) in room.iter().enumerate().rev() {
                if *room_occ == 0 {
                    return Some((room_idx, depth))
                }
                else if *room_occ != ((room_idx+1) as u8) {
                    break;
                }
            }
            None
        });

    // end_room: (room_idx, can_move_bottom_spot)
    let mut num_candidates: u8 = 0;
    for (idx, end_room) in candidate_end_rooms_i.enumerate() {
        candidate_end_rooms[idx] = end_room;
        num_candidates += 1;
    }

    // for each room which can be moved into
    // check for an amphipod which can directly move from another room

    // end_room: (room_idx, can_move_bottom_spot)
    for end_room_lookup_idx in 0..num_candidates {

        let (end_room_idx, end_room_depth_idx): (usize, usize) = candidate_end_rooms[end_room_lookup_idx as usize];
        let end_room_occupant: u8 = (end_room_idx+1) as u8;
        let end_room_hall_idx = ROOM_HALL_IDX_LOOKUP[end_room_idx];

        // move room -> room
        for (start_room_idx, start_room) in rooms.iter_mut().enumerate() {

            // start_room same as end_room, continue;
            if start_room_idx == end_room_idx { continue; }

            // check if top non-empty occupant of room matches end room desired occupant
            let mut start_depth_opt: Option<usize> = None;
            for (depth_idx, occ) in start_room.iter().enumerate() {
                if *occ == end_room_occupant {
                    start_depth_opt = Some(depth_idx);
                    break;
                }
                // occ == invalid occupant
                else if *occ != 0 {
                    break;
                }
            }

            // if occupant matching end_room found:
            // if valid_start_found {
            if let Some(start_depth_idx) = start_depth_opt {

                // ROOM_HALL_IDX_LOOKUP[start_room_idx]..ROOM_HALL_IDX_LOOKUP[end_room_idx]
                let start_room_hall_idx = ROOM_HALL_IDX_LOOKUP[start_room_idx];

                // determine if unobstructed path exists start_room -> end_room
                let is_path_blocked: bool = 
                    hallway[(min(start_room_hall_idx, end_room_hall_idx) as usize)..((max(start_room_hall_idx, end_room_hall_idx)+1) as usize)]
                    .iter()
                    .any(|path_pos| *path_pos > 0);

                if !is_path_blocked {

                    start_room[start_depth_idx] = 0;
                    rooms[end_room_idx][end_room_depth_idx] = end_room_occupant;

                    // calculate cost
                    *cost += calc_move_cost(
                        end_room_occupant,
                        start_room_hall_idx, end_room_hall_idx,
                        (start_depth_idx+1) as u8, (end_room_depth_idx+1) as u8
                    );

                    return true;
                }                
            }
        }

        // move hall -> room
        // check all hallway positions
        for (hall_pos, hall_occupant) in hallway.iter().enumerate() {

            // check hall occupied at position and matches end_room desired occupant
            if *hall_occupant == ((end_room_idx+1) as u8) {

                // determine if unobstructed path exists hall_pos -> end_room
                // NOTE: hall_pos + 1, hall_pos is always occupied

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


fn attempt_move_to_hall(cur_state: ([u8; 11], [[u8; 2]; 4], u64),
    valid_spaces: &mut [u8; 7],
    state_stack: &mut Vec<([u8; 11], [[u8; 2]; 4], u64)>) {

    let (cur_hallway, cur_rooms, cur_cost) = (cur_state.0, cur_state.1, cur_state.2);

    // (room_idx, depth_idx)
    let can_move_to_hall_list = cur_rooms.iter()
        .enumerate()
        .filter_map(|(room_idx, room)| {

            // if any invalid non-empty occupants found:
            //     return depth of top-most non-empty occupant

            let room_intended_occ = (room_idx + 1) as u8;

            // room has a non-empty invalid occupant
            if room.iter().any(|occupant| *occupant != room_intended_occ && *occupant != 0) {
                // return top-most room occupant
                Some(
                    room.iter()
                        .enumerate()
                        .find_map(|(depth_idx, occ)|{
                            if *occ != 0 { Some((room_idx, depth_idx)) }
                            else { None }
                        })
                        .unwrap()
                )
            } else {
                None
            }
        });

    // Check for move from invalid room occupant into unoccupied hall space
    // Make sure not to allow moving past other amphipods
    // Do not allow moving into hall space over the entrance to a room

    // (room_idx, depth)
    for occupant in can_move_to_hall_list {
        let (occupant_room_idx, occupant_depth_idx): (usize, usize) = (occupant.0, occupant.1);

        // iter over hallway positions moving left & right

        let mut left_idx: i8 = (ROOM_HALL_IDX_LOOKUP[occupant_room_idx]-1) as i8;
        let mut right_idx: i8 = (ROOM_HALL_IDX_LOOKUP[occupant_room_idx]+1) as i8;

        let mut valid_space_idx: u8 = 0;

        while left_idx >= 0 || right_idx < (cur_hallway.len() as i8) {

            // range check and not above room check
            if left_idx >= 0 && (left_idx % 2 != 0 || left_idx == 0) {
                // if unoccupied
                if cur_hallway[left_idx as usize] == 0 {
                    valid_spaces[valid_space_idx as usize] = left_idx as u8;
                    valid_space_idx += 1;
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
                    valid_spaces[valid_space_idx as usize] = right_idx as u8;
                    valid_space_idx += 1;
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
        for space_idx in 0..valid_space_idx {
            let space = valid_spaces[space_idx as usize];

            let mut new_rooms = cur_rooms.clone();
            new_rooms[occupant_room_idx][occupant_depth_idx] = 0;

            let mut new_hallway = cur_hallway.clone();

            // set to value of occupant moving out of room
            let unit_moving = cur_rooms[occupant_room_idx][occupant_depth_idx];
            new_hallway[space as usize] = unit_moving;

            // calculate cost
            let new_cost: u64 = cur_cost + calc_move_cost(
                unit_moving,
                ROOM_HALL_IDX_LOOKUP[occupant_room_idx] as u8, space,
                (occupant_depth_idx+1) as u8, 0
            );

            state_stack.push((new_hallway, new_rooms, new_cost));
        }
    }
}

#[allow(dead_code)]
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
    println!("\n");
}


pub fn exec(src: &str, print: bool) {
    // let src: &str = include_str!("../input/d23.txt");
    let mut lines = src.lines().skip(2);

    let hallway: [u8; 11] = [0; 11];

    let mut rooms: [[u8; 2]; 4] = [[0; 2]; 4];

    let val_from_occupant = |occupant| {match occupant {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        _ => panic!("invalid amphipod!")
    }};

    // top-level room occupants
    for (room_idx, room_occupant) in lines.next().unwrap().trim().replace('#',"").chars().enumerate() {
        rooms[room_idx][0] = val_from_occupant(room_occupant);
    }

    // bottom-level room occupants
    for (room_idx, room_occupant) in lines.next().unwrap().trim().replace('#',"").chars().enumerate() {
        rooms[room_idx][1] = val_from_occupant(room_occupant);
    }

    // (hallway, rooms, cost)
    let mut state_stack: Vec<([u8; 11], [[u8; 2]; 4], u64)> = vec![(hallway, rooms, 0)];

    let mut min_cost: u64 = u64::MAX;

    let mut candidate_end_rooms: [(usize, usize); 4] = [(0, 0); 4];

    // 11 hallway spaces - 4 invalid spaces outside rooms
    let mut valid_spaces: [u8; 7] = [0; 7];

    while !state_stack.is_empty() {

        let cur_state = state_stack.pop().unwrap();

        let (mut cur_hallway, mut cur_rooms, mut cur_cost) = (cur_state.0, cur_state.1, cur_state.2);

        // path too expensive
        if cur_cost > min_cost {
            continue;
        }

        // check if state is complete: any rooms unfilled with correct occupants
        let is_not_done: bool = cur_rooms.iter().enumerate().any(|(room_idx, room)| {
            room.iter().any(|occupant| *occupant != ((room_idx+1) as u8))
        });


        if !is_not_done {
            min_cost = min(min_cost, cur_cost);
            continue;
        }


        // check for direct move to final location
        // if found, exec, push new state, and stop enumeration
        if attempt_direct_move(&mut cur_hallway,
            &mut cur_rooms,
            &mut candidate_end_rooms,
            &mut cur_cost) {
            state_stack.push((cur_hallway, cur_rooms, cur_cost));
            continue;
        }

        attempt_move_to_hall((cur_hallway, cur_rooms, cur_cost), &mut valid_spaces, &mut state_stack);
    }

    if print { println!("result: {}", min_cost) }
}