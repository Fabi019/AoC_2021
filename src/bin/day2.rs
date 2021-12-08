#![allow(dead_code)]

fn main() {
    let data = std::fs::read_to_string("assets/day2.txt").unwrap();
    //part_one(&data)
    part_two(&data)
}

fn part_one(data: &str) {
    let mut horizontal = 0;
    let mut depth = 0;
    data.lines().for_each(|l| {
        let cmd: Vec<&str> = l.split(' ').collect();
        let value: u32 = cmd[1].parse().unwrap();
        match cmd[0] {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => {}
        }
    });
    println!(
        "horizontal: {}, depth: {}, total: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn part_two(data: &str) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    data.lines().for_each(|l| {
        let cmd: Vec<&str> = l.split(' ').collect();
        let value: u32 = cmd[1].parse().unwrap();
        match cmd[0] {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => {}
        }
    });
    println!(
        "horizontal: {}, depth: {}, total: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
