use std::env;

use packet::compare_packets;
use regex::Regex;

use crate::packet::Packet;

mod packet;

enum ParseListState {
    Unknown,
    Number,
    List,
}

fn parse_list(s: &str) -> Packet {
    use ParseListState::*;
    let mut ps = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    let mut state = Unknown;
    let mut new_packet = String::new();
    let mut bc = 0;

    for i in 1..chars.len() {
        let c = chars[i];

        if matches!(state, Unknown) {
            match c {
                '[' => state = List,
                c if c.is_numeric() => state = Number,
                _ => {
                    panic!("Invalid State!")
                }
            }
        }

        if bc == 0 && (c == ',' || i == chars.len() - 1) {
            ps.push(parse_packet(new_packet.as_str()));
            state = Unknown;
            bc = 0;
            new_packet = String::new();
            continue;
        }

        if matches!(state, List) {
            if c == '[' {
                bc += 1;
            } else if c == ']' {
                bc -= 1;
            }

            new_packet.push(chars[i])
        }

        if matches!(state, Number) {
            new_packet.push(chars[i])
        }
    }

    return Packet::List(ps);
}

fn parse_packet(s: &str) -> Packet {
    let lit_re = Regex::new(r"^\d+$").unwrap();

    if lit_re.is_match(s) {
        return Packet::Number(s.parse().unwrap());
    }

    if s == "[]" {
        return Packet::List(Vec::new());
    }

    let arr_re = Regex::new(r"^\[.*\]$").unwrap();
    if arr_re.is_match(s) {
        return parse_list(s);
    }

    panic!("Invalid packet string.");
}

fn part_a() {
    let input = include_str!("./input.txt");
    let test_input = include_str!("./test-input.txt");

    let packet_pairs = input
        .split("\n\n")
        .map(|s| {
            let mut split = s.split('\n');
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(p1, p2)| (parse_packet(p1), parse_packet(p2)))
        .collect::<Vec<(Packet, Packet)>>();

    let sum = packet_pairs
        .iter()
        .map(|(p1, p2)| compare_packets(p1, p2))
        .enumerate()

        // .inspect(|(i, b)| println!("{}: {b}", i + 1));
        .filter(|&(_i, b)| b)
        .map(|(i, _b)| i+1)
        .sum::<usize>();

    println!("Sum {sum}");
}


fn part_b() {
    env::set_var("RUST_BACKTRACE", 1.to_string());
    let input = include_str!("./input.txt");
    let test_input = include_str!("./test-input.txt");

    let re_between_packets = Regex::new("\n+").unwrap();

    let mut packets = re_between_packets
        .split(input)
        .filter(|s| !s.is_empty())
        // .inspect(|s| println!("p: {s}"))
        .map(|s| parse_packet(s))
        .collect::<Vec<Packet>>();

    let divider_packets = (parse_packet("[[2]]"), parse_packet("[[6]]"));
    packets.push(divider_packets.0.clone());
    packets.push(divider_packets.1.clone());

    packets.sort_by(|p1, p2| p1.cmp(p2));

    packets.iter().for_each(|p| println!("{}", p.to_string()));

    let mut decoder_key = 1;

    for (i, p) in packets.iter().enumerate() {
        if p == &divider_packets.0 || p == &divider_packets.1 {
            decoder_key *= i + 1;
        }
    }

    println!("Decoder key: {decoder_key}");

}

fn main() {
    part_b()
}
