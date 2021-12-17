use crate::day16::data::take_bits;
use log::trace;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Packet {
    pub(super) version: u8,
    pub(super) data: Data,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum PacketType {
    Literal,
    Operator,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum Data {
    Literal(u128),
    Operator(OperatorData),
}

impl From<u8> for PacketType {
    fn from(x: u8) -> Self {
        match x {
            4 => PacketType::Literal,
            _ => PacketType::Operator,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Length {
    TotalBits(u64),
    SubPacketCount(u64),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct OperatorData {
    pub(super) operation: Operation,
    pub(super) length: Length,
    pub(super) sub_packets: Box<Vec<Packet>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl Packet {
    pub(super) fn packet_type(&self) -> PacketType {
        match self.data {
            Data::Literal(_) => PacketType::Literal,
            Data::Operator(_) => PacketType::Operator,
        }
    }

    /// Returns self and the "end" of the parse. That is, the index of the last bit that was parsed
    /// plus one. So if `first_bit` is 0, and three bits were parsed, returns `(Self, 4)`. Another
    /// way to look at it is, returns the next bit index that should be parsed.
    pub(super) fn parse(bits: &[u64], first_bit: usize) -> (Self, usize) {
        let mut current_range = first_bit..first_bit + 3;
        let version = take_bits(bits, current_range.clone()).unwrap() as u8;
        current_range = current_range.end..current_range.end + 3;
        let packet_type_value = take_bits(bits, current_range.clone()).unwrap() as u8;
        let packet_type = PacketType::from(packet_type_value);
        let data = match packet_type {
            PacketType::Literal => {
                let (literal, end) = parse_literal(bits, current_range.end);
                current_range.start = end;
                current_range.end = end;
                Data::Literal(literal)
            }
            PacketType::Operator => {
                let (operator_data, end) =
                    parse_operator_data(bits, current_range.end, packet_type_value);
                current_range.start = end;
                current_range.end = end;
                Data::Operator(operator_data)
            }
        };
        (Self { version, data }, current_range.end)
    }

    /// Packets with type ID 0 are sum packets - their value is the sum of the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
    /// Packets with type ID 1 are product packets - their value is the result of multiplying together the values of their sub-packets. If they only have a single sub-packet, their value is the value of the sub-packet.
    /// Packets with type ID 2 are minimum packets - their value is the minimum of the values of their sub-packets.
    /// Packets with type ID 3 are maximum packets - their value is the maximum of the values of their sub-packets.
    /// Packets with type ID 5 are greater than packets - their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
    /// Packets with type ID 6 are less than packets - their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
    /// Packets with type ID 7 are equal to packets - their value is 1 if the value of the first sub-packet is equal to
    pub(super) fn evaluate(&self) -> u64 {
        match &self.data {
            Data::Literal(literal) => *literal as u64,
            Data::Operator(o) => match o.operation {
                Operation::Sum => {
                    let mut sum = 0;
                    for sub_packet in o.sub_packets.iter() {
                        sum += sub_packet.evaluate();
                    }
                    sum
                }
                Operation::Product => {
                    if o.sub_packets.is_empty() {
                        0
                    } else {
                        let mut product = 1;
                        for sub_packet in o.sub_packets.iter() {
                            product *= sub_packet.evaluate();
                        }
                        product
                    }
                }
                Operation::Min => {
                    let mut min = u64::MAX;
                    for sub_packet in o.sub_packets.iter() {
                        let current = sub_packet.evaluate();
                        if current < min {
                            min = current;
                        }
                    }
                    min
                }
                Operation::Max => {
                    let mut max = u64::MIN;
                    for sub_packet in o.sub_packets.iter() {
                        let current = sub_packet.evaluate();
                        if current > max {
                            max = current;
                        }
                    }
                    max
                }
                Operation::Greater => {
                    let a = o.sub_packets.get(0).unwrap().evaluate();
                    let b = o.sub_packets.get(1).unwrap().evaluate();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                Operation::Less => {
                    let a = o.sub_packets.get(0).unwrap().evaluate();
                    let b = o.sub_packets.get(1).unwrap().evaluate();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                Operation::Equal => {
                    let a = o.sub_packets.get(0).unwrap().evaluate();
                    let b = o.sub_packets.get(1).unwrap().evaluate();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn parse_literal(bits: &[u64], mut start: usize) -> (u128, usize) {
    let mut literal = 0;
    loop {
        let these_bits = take_bits(bits, start..start + 5).unwrap();
        trace!("{:#064b} - these_bits", these_bits);
        start += 5;
        let more = these_bits & 0b10000 == 0b10000;
        let this_value = these_bits & 0b1111;
        trace!("{:#064b} - this_value", this_value);
        literal <<= 4;
        trace!("{:#064b} - literal shifted", literal);
        literal |= this_value as u128;
        trace!("{:#064b} - literal", literal);
        if !more {
            break;
        }
    }
    // while let Some(value) = take_bits(bits, start..start + 1) {
    //     if value != 0 {
    //         break;
    //     }
    //     start += 1;
    // }
    (literal, start)
}

fn parse_operator_data(bits: &[u64], mut start: usize, o: u8) -> (OperatorData, usize) {
    let operation = match o {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Min,
        3 => Operation::Max,
        5 => Operation::Greater,
        6 => Operation::Less,
        7 => Operation::Equal,
        _ => panic!("Bad operation {}", o),
    };
    let length_bit = take_bits(bits, start..start + 1).unwrap();
    start += 1;
    let length = if length_bit == 0 {
        // 15 bit number
        let bit_count = take_bits(bits, start..start + 15).unwrap();
        start += 15;
        Length::TotalBits(bit_count)
    } else {
        // 11 bit count
        let sub_packet_count = take_bits(bits, start..start + 11).unwrap();
        start += 11;
        Length::SubPacketCount(sub_packet_count)
    };
    let mut sub_packets = Vec::new();
    match length {
        Length::TotalBits(bit_count) => {
            //
            let end = start + bit_count as usize;
            while start < end {
                let (packet, new_start) = Packet::parse(bits, start);
                start = new_start;
                sub_packets.push(packet);
            }
            start = end;
        }
        Length::SubPacketCount(count) => {
            for _ in 0..count {
                let (packet, new_start) = Packet::parse(bits, start);
                start = new_start;
                sub_packets.push(packet);
            }
        }
    }
    (
        OperatorData {
            operation,
            length,
            sub_packets: Box::new(sub_packets),
        },
        start,
    )
}

#[cfg(test)]
mod test {
    use crate::day16::data::{parse_hex, TEST_1, TEST_2, TEST_3};
    use crate::day16::packet::{parse_literal, Data, Packet, PacketType};

    #[test]
    fn test_1() {
        let bits = parse_hex(TEST_1);
        let (packet, _) = Packet::parse(&bits, 0);
        assert_eq!(packet.version, 6);
        assert!(matches!(packet.packet_type(), PacketType::Literal));
        match &packet.data {
            Data::Literal(literal) => assert_eq!(*literal, 2021),
            _ => panic!("wrong data: {:?}", packet.data),
        }
    }

    #[test]
    fn test_2() {
        let bits = parse_hex(TEST_2);
        let (packet, _) = Packet::parse(&bits, 0);
        assert_eq!(packet.version, 1);
        assert!(matches!(packet.packet_type(), PacketType::Operator));
        let operator_data = match packet.data {
            Data::Literal(_) => panic!("wrong type"),
            Data::Operator(o) => o,
        };
        assert_eq!(operator_data.sub_packets.len(), 2);
    }

    #[test]
    fn test_3() {
        let bits = parse_hex(TEST_3);
        let (packet, _) = Packet::parse(&bits, 0);
        assert_eq!(packet.version, 7);
        assert!(matches!(packet.packet_type(), PacketType::Operator));
        let operator_data = match packet.data {
            Data::Literal(_) => panic!("wrong type"),
            Data::Operator(o) => o,
        };
        assert_eq!(operator_data.sub_packets.len(), 3);
    }

    #[test]
    fn test_4() {
        let bits = parse_hex("620080001611562C8802118E34");
        let (packet, _) = Packet::parse(&bits, 0);
        assert_eq!(packet.version, 3);
        assert!(matches!(packet.packet_type(), PacketType::Operator));
        let operator_data = match packet.data {
            Data::Literal(_) => panic!("wrong type"),
            Data::Operator(o) => o,
        };
        assert_eq!(operator_data.sub_packets.len(), 2);
    }

    #[test]
    fn test_literal() {
        let bits = parse_hex(TEST_1);
        let (literal, end) = parse_literal(&bits, 6);
        assert_eq!(literal, 2021);
        assert_eq!(end, 21);
    }
}
