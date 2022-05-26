use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Clone, PartialEq)]
enum LengthType {
    TotalBitLength(u16),
    SubPacketNum(u16),
}
#[derive(Debug)]
struct OperatorPacket {
    // header
    version: u8,
    len_type: LengthType,

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

    let mut version_sum: u64 = 0;

    while !bin_data.is_empty() {
        match parse_state {
            ParseState::PacketHeader => {

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

                    if packet_satisfied {
                        incr_parent_subpackets_read(&mut packet_stack);
                        continue;
                    } else {
                        packet_stack.push(Rc::clone(&parent_packet_ref));
                    }
                }

                more_to_read = is_root_packet || !packet_stack.is_empty();
                if !more_to_read { break; }

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

                version_sum += version as u64;

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
                let mut _literal_value: u64 = 0;

                while !is_last_block {
                    is_last_block = bin_data.pop().unwrap() == '0';
                    for _ in 0..4 {
                        _literal_value <<= 1;
                        _literal_value += bin_data.pop().unwrap().to_string().parse::<u8>().unwrap() as u64;
                    }
                    bits_read += 5;
                }

                parse_state = ParseState::PacketHeader;

                // if the parent operator packet has a length constraint based on num packets
                // increment parent.subpackets_read
                incr_parent_subpackets_read(&mut packet_stack);
            },
        }
    }

    println!("result: {}", version_sum);
}