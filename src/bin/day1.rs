#![allow(dead_code)]

fn main() {
    let data = std::fs::read_to_string("assets/day1.txt").unwrap();
    //part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let lines: Vec<u32> = data.lines().map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    let mut previous = lines[0];
    for &current in lines.iter().skip(1) {
        if previous < current {
            count += 1;
            println!("{} increased", current);
        } else {
            println!("{} decreased", current);
        }
        previous = current;
    }
    println!("Total increased: {}", count); // answer is 1521
}

fn part_two(data: &str) {
    let lines: Vec<u32> = data.lines().map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    let mut previous = lines[0] + lines[1] + lines[2];
    for i in 1..lines.len() - 2 {
        let current = lines[i] + lines[i + 1] + lines[i + 2];
        if previous < current {
            count += 1;
            println!("{} increased", current);
        } else {
            println!("{} decreased", current);
        }
        previous = current;
    }
    println!("Total increased: {}", count); // answer is 1543
}
