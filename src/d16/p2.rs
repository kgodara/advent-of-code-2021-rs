use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Clone, PartialEq)]
enum LengthType {
    TotalBitLength(u16),
    SubPacketNum(u16),
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum OperatorType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum  = 3,
    Gt = 5,
    Lt = 6,
    Equal = 7,
}

#[derive(Debug)]
struct OperatorPacket {
    // header
    version: u8,
    len_type: LengthType,

    op_type: OperatorType,
    values: Vec<u64>,

    start_bit_num: u64,
    subpackets_read: u64,
}

enum ParseState {
    PacketHeader,
    LiteralValue,
}

fn incr_parent_subpackets_read(packet_stack: &mut Vec<Rc<RefCell<OperatorPacket>>>) {
    if !packet_stack.is_empty() {
        let parent_packet_ref = packet_stack.pop().unwrap();
        let parent_packet_len_type = &parent_packet_ref.borrow().len_type.clone();

        if let LengthType::SubPacketNum(_) = parent_packet_len_type {
            parent_packet_ref.borrow_mut().subpackets_read += 1;
        }
        packet_stack.push(Rc::clone(&parent_packet_ref));
    }
}

fn add_value_to_parent(packet_stack: &mut Vec<Rc<RefCell<OperatorPacket>>>, value: u64) {
    if !packet_stack.is_empty() {
        let parent_packet_ref = packet_stack.pop().unwrap();
        parent_packet_ref.borrow_mut().values.push(value);
        packet_stack.push(Rc::clone(&parent_packet_ref));
    }
}

fn calc_packet_value(packet: &Rc<RefCell<OperatorPacket>>) -> u64 {

    let packet_op_type = packet.borrow().op_type;
    let packet_values = &packet.borrow().values.clone();

    match packet_op_type {
        OperatorType::Sum => {
            packet_values.iter().sum()
        },
        OperatorType::Product => {
            if packet_values.len() == 1 {
                packet_values[0]
            } else {
                packet_values.iter().cloned().reduce(|acc, elem| acc * elem).unwrap()
            }
        },
        OperatorType::Minimum => {
            *packet_values.iter().min().unwrap()
        },
        OperatorType::Maximum => {
            *packet_values.iter().max().unwrap()
        },
        OperatorType::Gt => {
            if packet_values[0] > packet_values[1] { 1 } else { 0 }
        },
        OperatorType::Lt => {
            if packet_values[0] < packet_values[1] { 1 } else { 0 }
        },
        OperatorType::Equal => {
            if packet_values[0] == packet_values[1] { 1 } else { 0 }
        },
    }
}


pub fn exec(src: String) {

    let mut bin_data: Vec<char> = vec![];

    for line in src.lines() {
        for ch in line.chars() {
            let hex_val: u8 = u8::from_str_radix(&ch.to_string(), 16).unwrap();
            bin_data.push( char::from_digit(((hex_val>>3) & 1) as u32, 10).unwrap() );
            bin_data.push( char::from_digit(((hex_val>>2) & 1) as u32, 10).unwrap() );
            bin_data.push( char::from_digit(((hex_val>>1) & 1) as u32, 10).unwrap() );
            bin_data.push( char::from_digit((hex_val & 1) as u32, 10).unwrap() );
        }
    }

    bin_data = bin_data.into_iter().rev().collect();

    let mut parse_state: ParseState = ParseState::PacketHeader;

    let mut is_root_packet: bool = true;
    let mut more_to_read: bool;

    let mut bits_read: u64 = 0;

    let mut packet_stack: Vec<Rc<RefCell<OperatorPacket>>> = Vec::new();

    let mut latest_value: u64 = 0;

    while !bin_data.is_empty() {
        match parse_state {
            ParseState::PacketHeader => {
                // stack empty and not root packet - break
                if !is_root_packet && packet_stack.is_empty() {
                    break;
                }
                // check if cur_packet has more subpackets
                // if true: parse next subpacket
                // else: set cur_packet = cur_packet.parent and continue

                if !is_root_packet {

                    let parent_packet_ref = packet_stack.pop().unwrap();
                    let parent_packet_len_type = &parent_packet_ref.borrow().len_type;
                    
                    let packet_satisfied: bool = match parent_packet_len_type {
                        LengthType::SubPacketNum(subpacket_num) => {
                            // No more subpackets within this parent packet
                            parent_packet_ref.borrow().subpackets_read >= *subpacket_num as u64
                        },
                        LengthType::TotalBitLength(bit_len) => {
                            // No more subpackets within this parent packet
                            (bits_read - parent_packet_ref.borrow().start_bit_num) >= *bit_len as u64
                        }
                    };

                    // packet has been read completely
                    if packet_satisfied {
                        incr_parent_subpackets_read(&mut packet_stack);
    
                        // Calculate value of this operator packet and add to parent operator packet
                        latest_value = calc_packet_value(&parent_packet_ref);
    
                        add_value_to_parent(&mut packet_stack, latest_value);
    
                        continue;
                    } else {
                        packet_stack.push(Rc::clone(&parent_packet_ref));
                    }
                }

                more_to_read = is_root_packet || !packet_stack.is_empty();
                // break if no more to read
                if !more_to_read { 
                    break;
                }

                is_root_packet = false;

                let mut version: u8 = 0;

                for _ in 0..3 {
                    version <<= 1;
                    version += &bin_data.pop().unwrap().to_string().parse::<u8>().unwrap();
                }

                let mut packet_type_id: u8 = 0;

                for _ in 0..3 {
                    packet_type_id <<= 1;
                    packet_type_id += &bin_data.pop().unwrap().to_string().parse::<u8>().unwrap();
                }

                // include header bits
                bits_read += 6;

                if packet_type_id == 4 {
                    parse_state = ParseState::LiteralValue;
                } else {
                    let len_type: LengthType = 
                        // total bit length (15 bits)
                        if bin_data.pop().unwrap() == '0' {
                            let mut total_bit_len: u16 = 0;
                            for _ in 0..15 {
                                total_bit_len <<= 1;
                                total_bit_len += bin_data.pop().unwrap().to_string().parse::<u16>().unwrap();
                            }
                            bits_read += 15;
                            LengthType::TotalBitLength(total_bit_len)
                        }
                        // number of subpackets contained (11 bits)
                        else {
                            let mut subpacket_num: u16 = 0;
                            for _ in 0..11 {
                                subpacket_num <<= 1;
                                subpacket_num += bin_data.pop().unwrap().to_string().parse::<u16>().unwrap();
                            }
                            bits_read += 11;
                            LengthType::SubPacketNum(subpacket_num)
                        };

                    // for length type identifier
                    bits_read += 1;
                        
                    packet_stack.push(Rc::new(
                        RefCell::new(
                            OperatorPacket {
                                version,
                                len_type,

                                op_type: unsafe { std::mem::transmute(packet_type_id as u8) },
                                values: vec![],

                                start_bit_num: bits_read,
                                subpackets_read: 0,
                            }
                        )
                    ));
                    parse_state = ParseState::PacketHeader;
                }
            },
            ParseState::LiteralValue => {
                // parse 5 bits at a time until the last 5 bit block parsed
                let mut is_last_block: bool = false;
                let mut literal_value: u64 = 0;

                while !is_last_block {
                    is_last_block = bin_data.pop().unwrap() == '0';
                    for _ in 0..4 {
                        literal_value <<= 1;
                        literal_value += bin_data.pop().unwrap().to_string().parse::<u8>().unwrap() as u64;
                    }
                    bits_read += 5;
                }

                parse_state = ParseState::PacketHeader;

                // if the parent operator packet has a length constraint based on num packets
                // increment parent.subpackets_read
                incr_parent_subpackets_read(&mut packet_stack);

                // Add to 'values' of parent operator packet
                add_value_to_parent(&mut packet_stack, literal_value);
            },
        }
    }

    // if there were no extra bits, value of root packet would not be calculated
    if !packet_stack.is_empty() {
        let parent_packet_ref = packet_stack.pop().unwrap();
        latest_value = calc_packet_value(&parent_packet_ref);
    }

    println!("result: {}", latest_value);
}