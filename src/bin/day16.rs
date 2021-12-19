use bitreader::BitReader;
use hex;
use std::cmp::{max, min};

const LITERAL: u8 = 4;

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    // 3 bits
    version: u8,
    // 3 bits
    typeId: u8,
    // packet data
    literal: u64,
    // sub-packets
    packets: Vec<Box<Packet>>,
}

impl Packet {
    fn new() -> Packet {
        Packet {
            version: 0,
            typeId: 0,
            literal: 0,
            packets: Vec::new(),
        }
    }

    fn _read_literal(&mut self, bits: &mut BitReader) {
        // Start peeling off 5-bit chunks until the first bit is 0
        let mut chunks = 0;
        for idx in 0..16 {
            let chunk = bits.read_u64(5).unwrap();
            self.literal += (chunk & 0b1111) << ((15 - idx) * 4);
            if chunk & 16 == 0 {
                chunks = idx;
                break;
            }
        }
        // Now shift right by the chunks we didn't see
        self.literal >>= (15 - chunks) * 4;
    }

    fn _read_operator(&mut self, bits: &mut BitReader) {}

    fn read(bits: &mut BitReader) -> Packet {
        let mut packet = Packet::new();

        packet.version = bits.read_u8(3).unwrap();
        packet.typeId = bits.read_u8(3).unwrap();

        if packet.typeId == LITERAL {
            packet._read_literal(bits);
        } else {
            let how_far_kind = bits.read_u8(1).unwrap();
            let (mut num_bits, mut num_packets) = (0, 0);
            if how_far_kind == 0 {
                // 15 bits for length of packet data
                num_bits = bits.read_u16(15).unwrap();
            } else {
                // 11 bits for number of packets
                num_packets = bits.read_u16(11).unwrap();
            }
            // Read packets until we satisfy the constraint
            let start_pos = bits.position();
            loop {
                if (how_far_kind == 0 && (bits.position() - start_pos) >= num_bits as u64)
                    || (how_far_kind == 1 && num_packets == 0)
                {
                    break;
                }
                // Read a packet out of the relative reader
                packet.packets.push(Box::new(Packet::read(bits)));
                if how_far_kind == 1 {
                    num_packets -= 1;
                }
            }
        }

        packet
    }

    fn sum_versions(&self) -> u32 {
        let mut rv = 0;

        for packet in &self.packets {
            rv += packet.sum_versions();
        }

        rv + self.version as u32
    }

    fn calculate(&self) -> u64 {
        let mut values: Vec<u64> = self.packets.iter().map(|p| p.calculate()).collect();

        if values.len() == 1 {
            // Shortcut: we're guaranteed to have two values for several types, and any type
            // that could possibly only have one would just return it
            return values[0];
        }

        let iter = values.iter();

        match self.typeId {
            0 => iter.sum(),
            1 => iter.fold(1, |a, b| a * b),
            2 => *iter.min().unwrap(),
            3 => *iter.max().unwrap(),
            4 => self.literal,
            5 => {
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("unknown type {}", self.typeId),
        }
    }
}

fn parse_input(input: &str) -> Packet {
    let hexed = hex::decode(input.as_bytes()).unwrap();
    let mut bits = BitReader::new(&hexed);
    Packet::read(&mut bits)
}

fn part_one(input: &str) -> u32 {
    parse_input(input).sum_versions()
}

fn part_two(input: &str) -> u64 {
    parse_input(input).calculate()
}

fn main() {
    let input = include_str!("day16.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Packet};

    #[test]
    fn it_works() {
        assert_eq!(
            parse_input("D2FE28"),
            Packet {
                version: 6,
                typeId: 4,
                literal: 2021,
                packets: Vec::new(),
            }
        );
        assert_eq!(
            parse_input("38006F45291200"),
            Packet {
                version: 1,
                typeId: 6,
                literal: 0,
                packets: Vec::from([
                    Box::new(Packet {
                        version: 6,
                        typeId: 4,
                        literal: 10,
                        packets: Vec::new(),
                    }),
                    Box::new(Packet {
                        version: 2,
                        typeId: 4,
                        literal: 20,
                        packets: Vec::new(),
                    })
                ])
            }
        );
    }
}
