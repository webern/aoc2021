/*!

!*/

mod data;
mod packet;

use crate::day16::data::{parse_hex, INPUT_DATA};
use crate::day16::packet::{Data, Packet};

pub fn solve() {
    println!("Part 1: {}", solve_part_1(INPUT_DATA));
    println!("Part 2: {}", solve_part_2(INPUT_DATA));
}

fn solve_part_1(input: &str) -> usize {
    let bits = parse_hex(input);
    let (packet, _) = Packet::parse(&bits, 0);
    let sum = version_sum(&packet);
    sum as usize
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.data {
            Data::Literal(_) => 0,
            Data::Operator(o) => {
                let mut op_sum = 0;
                for p in o.sub_packets.iter() {
                    op_sum += version_sum(p);
                }
                op_sum
            }
        }
}

fn solve_part_2(input: &str) -> usize {
    let bits = parse_hex(input);
    let (packet, _) = Packet::parse(&bits, 0);
    packet.evaluate() as usize
}
