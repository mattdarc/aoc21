// D2FE28
// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCCXXX
//
// VVV: packet version == 6
// TTT: type ID == 4
// AAAAA: data prefixed with 1 (not last)
// BBBBB: data prefixed with 1 (not last)
// CCCCC: data prefixed with 0 (last)
// XXX: ignored bits (after last group)
//
// All other type IDs are operators
// - length type ID is 0 -> next 15 bits are a number that represents the total length in bits of
// the sub-packets contained by this packet.
//
// - length type ID is 1 -> next 11 bits are a number that represents the number of sub-packets
// immediately contained by this packet.
//
// 00111000000000000110111101000101001010010001001000000000
// VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBBXXXXXXX
//
// VVV: packet version == 1
// TTT: type ID == 6
// I:   length type ID == 0
// L(15): length of sub-packets == 27 bits
// A(11): first sub-packet == 10 (broken down like above)
// A(16): second sub-packet == 20
//
// Parsing stops afterr length is reached (27).

use std::fmt::Write;

const TYPE_SUM: i64 = 0;
const TYPE_PRODUCT: i64 = 1;
const TYPE_MINIMUM: i64 = 2;
const TYPE_MAXIMUM: i64 = 3;
const TYPE_LITERAL: i64 = 4;
const TYPE_GREATER_THAN: i64 = 5;
const TYPE_LESS_THAN: i64 = 6;
const TYPE_EQUAL_TO: i64 = 7;

const LEN_TOTAL_LENGTH: i64 = 0;
const LEN_NUM_SUBPACKETS: i64 = 1;

struct BitStream(Vec<bool>);
impl BitStream {
    fn from_vec(stream: Vec<bool>) -> Self {
        BitStream(stream)
    }

    fn inner(&self) -> &[bool] {
        &self.0
    }
}

impl std::fmt::Debug for BitStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for &b in &self.0 {
            if b {
                f.write_char('1')?;
            } else {
                f.write_char('0')?;
            }
        }
        f.write_char('\n')
    }
}

fn to_integer(b: &[bool]) -> i64 {
    b.iter()
        .fold(0, |acc, &bit| (acc << 1) | if bit { 1 } else { 0 })
}

#[derive(Debug)]
enum PacketData {
    Literal(i64),
    Packets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: i64,
    type_id: i64,
    data: PacketData,
}

impl Packet {
    fn literal(&self) -> i64 {
        match &self.data {
            &PacketData::Literal(v) => v,
            &PacketData::Packets(_) => panic!("Called literal on a composite packet!"),
        }
    }

    fn packets(&self) -> &[Packet] {
        match &self.data {
            PacketData::Literal(_) => panic!("Called packets on a non-composite packet!"),
            PacketData::Packets(packets) => &packets,
        }
    }
}

fn hex_to_bits(hex: char) -> Vec<bool> {
    let num = hex.to_digit(16).expect("Invalid hex");
    (0..4).rev().map(|bit| (num & (1 << bit)) != 0).collect()
}

fn parse_literal(bits: &[bool]) -> (usize, i64) {
    let next = (0..)
        .enumerate()
        .step_by(5)
        .find(|b| !bits[b.1])
        .map(|(i, _)| i)
        .unwrap()
        + 5;

    let literal = bits[..next]
        .chunks_exact(5)
        .map(|c| to_integer(&c[1..]))
        .fold(0, |acc, num| (acc << 4) | num);

    (next, literal)
}

fn parse_n_bits(bits: &[bool], packet_start: usize, n_bits: usize) -> (usize, PacketData) {
    let mut next_packet = packet_start;

    let mut packets = Vec::new();
    while next_packet - packet_start < n_bits {
        let (i, packet) = parse_packet(&bits[next_packet..]);
        packets.push(packet);
        next_packet += i;
    }
    (next_packet, PacketData::Packets(packets))
}

fn parse_n_packets(bits: &[bool], packet_start: usize, n_packets: usize) -> (usize, PacketData) {
    let mut next_packet = packet_start;

    let mut packets = Vec::new();
    for _ in 0..n_packets {
        let (i, packet) = parse_packet(&bits[next_packet..]);
        packets.push(packet);
        next_packet += i;
    }
    (next_packet, PacketData::Packets(packets))
}

fn parse_packet(bits: &[bool]) -> (usize, Packet) {
    let version = to_integer(&bits[0..3]);
    let type_id = to_integer(&bits[3..6]);
    let (next, data) = if type_id == TYPE_LITERAL {
        let (next, literal) = parse_literal(&bits[6..]);
        (6 + next, PacketData::Literal(literal))
    } else {
        let length_id = to_integer(&bits[6..7]);
        if length_id == LEN_TOTAL_LENGTH {
            // Total length is the next 15 bits
            let num_bits = to_integer(&bits[7..22]) as usize;
            parse_n_bits(bits, 22, num_bits)
        } else {
            // Total number of sub-packets is the next 11
            assert_eq!(length_id, LEN_NUM_SUBPACKETS);
            let num_packets = to_integer(&bits[7..18]) as usize;
            parse_n_packets(bits, 18, num_packets)
        }
    };

    (
        next,
        Packet {
            version,
            type_id,
            data,
        },
    )
}

fn sum_packet_versions(packet: &Packet) -> i64 {
    packet.version as i64
        + match &packet.data {
            PacketData::Literal(_) => 0,
            PacketData::Packets(packets) => packets.iter().map(sum_packet_versions).sum(),
        }
}

fn process_packet(packet: &Packet) -> i64 {
    match packet.type_id {
        TYPE_SUM => packet.packets().iter().map(process_packet).sum(),
        TYPE_PRODUCT => packet.packets().iter().map(process_packet).product(),
        TYPE_MINIMUM => packet.packets().iter().map(process_packet).min().unwrap(),
        TYPE_MAXIMUM => packet.packets().iter().map(process_packet).max().unwrap(),
        TYPE_LITERAL => packet.literal(),
        TYPE_GREATER_THAN => {
            let packets = packet.packets();
            assert_eq!(packets.len(), 2);
            if process_packet(&packets[0]) > process_packet(&packets[1]) {
                1
            } else {
                0
            }
        }
        TYPE_LESS_THAN => {
            let packets = packet.packets();
            assert_eq!(packets.len(), 2);
            if process_packet(&packets[0]) < process_packet(&packets[1]) {
                1
            } else {
                0
            }
        }
        TYPE_EQUAL_TO => {
            let packets = packet.packets();
            assert_eq!(packets.len(), 2);
            if process_packet(&packets[0]) == process_packet(&packets[1]) {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

#[aoc_generator(day16)]
fn bits(input: &str) -> BitStream {
    BitStream::from_vec(input.chars().flat_map(hex_to_bits).collect())
}

#[aoc(day16, part1)]
fn part1(bits: &BitStream) -> i64 {
    let (_, root_packet) = parse_packet(bits.inner());
    sum_packet_versions(&root_packet)
}

#[aoc(day16, part2)]
fn part2(bits: &BitStream) -> i64 {
    let (_, root_packet) = parse_packet(bits.inner());
    process_packet(&root_packet)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(part1(&bits(r"D2FE28")), 6);
        assert_eq!(part1(&bits(r"EE00D40C823060")), 14);
    }

    #[test]
    fn example() {
        assert_eq!(part1(&bits(r"8A004A801A8002F478")), 16);
        assert_eq!(part2(&bits("9C0141080250320F1802104A08")), 1);
        assert_eq!(part2(&bits("C200B40A82")), 3);
        assert_eq!(part2(&bits("04005AC33890")), 54);
        assert_eq!(part2(&bits("880086C3E88112")), 7);
        assert_eq!(part2(&bits("CE00C43D881120")), 9);
        assert_eq!(part2(&bits("D8005AC2A8F0")), 1);
    }

    #[test]
    fn example2() {
        let input = bits(r"620080001611562C8802118E34");
        assert_eq!(part1(&input), 12);
        //assert_eq!(part2(&input), 315);
    }

    #[test]
    fn example3() {
        let input = bits(r"C0015000016115A2E0802F182340");
        assert_eq!(part1(&input), 23);
        //assert_eq!(part2(&input), 315);
    }

    #[test]
    fn example4() {
        let input = bits(r"A0016C880162017C3686B18A3D4780");
        assert_eq!(part1(&input), 31);
        //assert_eq!(part2(&input), 315);
    }
}
