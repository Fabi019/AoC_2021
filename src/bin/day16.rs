#![feature(exact_size_is_empty)]

use std::vec::IntoIter;

fn main() {
    let data = std::fs::read_to_string("assets/day16.txt").unwrap();
    let decoded = data
        .chars()
        .map(|c| format!("{:0>4b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .map(|s| s.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();
    dbg!(decode_packet(&mut decoded.into_iter()));
}

fn decode_packet(data: &mut IntoIter<char>) -> (u32, u64) {
    let version = u32::from_str_radix(&data.take(3).collect::<String>(), 2).unwrap();
    let type_id = u32::from_str_radix(&data.take(3).collect::<String>(), 2).unwrap();
    println!("Decoding packet: version = {}, type = {}", version, type_id);
    return if type_id == 4 {
        let literal = decode_literal(data);
        println!("Value of literal packet: {}", literal);
        (version, literal)
    } else {
        let packets = decode_operator(data);
        println!("Value of operator packet: {:?}", packets);
        let total_version = version + packets.iter().map(|e| e.0).sum::<u32>();
        let result = match type_id {
            0 => packets.iter().map(|e| e.1).sum(),
            1 => packets.iter().map(|e| e.1).product(),
            2 => packets.iter().map(|e| e.1).min().unwrap(),
            3 => packets.iter().map(|e| e.1).max().unwrap(),
            5 => (packets[0].1 > packets[1].1) as u64,
            6 => (packets[0].1 < packets[1].1) as u64,
            7 => (packets[0].1 == packets[1].1) as u64,
            _ => 0
        };
        (total_version, result)
    };
}

fn decode_literal(data: &mut IntoIter<char>) -> u64 {
    let mut total_number = String::new();
    loop {
        let exit = data.next().unwrap() == '0';
        let chunk = data.take(4);
        total_number.push_str(&chunk.collect::<String>());
        if exit {
            break;
        }
    }
    println!("Literal packet: {}", total_number);
    u64::from_str_radix(&total_number, 2).unwrap()
}

fn decode_operator(data: &mut IntoIter<char>) -> Vec<(u32, u64)> {
    let mut sub_packets = Vec::new();
    let len_type = data.next().unwrap() == '0';
    if len_type {
        let bytes_len = u64::from_str_radix(&data.take(15).collect::<String>(), 2).unwrap();
        let mut next = data
            .take(bytes_len as usize)
            .collect::<Vec<char>>()
            .into_iter();
        while !next.is_empty() {
            sub_packets.push(decode_packet(&mut next));
        }
    } else {
        let packet_count = u64::from_str_radix(&data.take(11).collect::<String>(), 2).unwrap();
        for _ in 0..packet_count {
            sub_packets.push(decode_packet(data));
        }
    }
    sub_packets
}
