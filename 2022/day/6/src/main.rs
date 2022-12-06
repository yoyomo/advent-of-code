use std::{collections::VecDeque, fs};

fn start_of(packet: String, START_OF_SIZE: usize) {
    let mut start_of_packet: VecDeque<char> = VecDeque::with_capacity(START_OF_SIZE);
    for (i, character) in packet.char_indices() {
        if start_of_packet.len() == START_OF_SIZE {
            start_of_packet.pop_front();
        }

        start_of_packet.push_back(character);

        if start_of_packet.len() == START_OF_SIZE {
            if !start_of_packet.iter().any(|&c| start_of_packet.iter().filter(|&z| c == *z ).count() > 1){
                println!("start at {} with {:?}", i+1, start_of_packet);
                break;
            }
        }
    }
}

fn main() {
    let packet = fs::read_to_string("data/input.txt").expect("");

    start_of(packet, 4);
}
