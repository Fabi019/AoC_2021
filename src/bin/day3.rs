use std::collections::BTreeMap;
use std::iter::Map;
use std::ops::Add;

fn main() {
    let data = std::fs::read_to_string("assets/day3.txt").unwrap();
    //part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut numbers = BTreeMap::new();
    data.lines().for_each(|l| {
        for (pos, c) in l.chars().enumerate() {
            numbers
                .entry(pos as u32)
                .or_insert(Vec::new())
                .push(c.to_digit(2).unwrap())
        }
    });

    let mut gamma = String::with_capacity(12);
    let mut epsilon = String::with_capacity(12);

    for (pos, nums) in numbers.iter() {
        let sum = nums.iter().sum::<u32>();
        let total = nums.len();
        let most = if sum > (total / 2) as u32 { 1 } else { 0 };
        let least = 1 - most;
        gamma.push_str(&most.to_string());
        epsilon.push_str(&least.to_string());
        println!("Pos: {}, Most: {}, Least: {}", pos, most, least);
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    println!(
        "Gamma: {}, Epsilon: {}, Total: {}",
        gamma,
        epsilon,
        gamma * epsilon
    )
}

fn part_two(data: &str) {
    const LINE_SIZE: usize = 12;

    let mut numbers = BTreeMap::new();
    for (pos, line) in data.lines().enumerate() {
        for c in line.chars() {
            numbers
                .entry(pos)
                .or_insert(Vec::new())
                .push(c.to_digit(2).unwrap());
        }
    }

    let mut oxygen: Vec<Vec<u32>> = numbers.values().cloned().collect();
    let mut co2: Vec<Vec<u32>> = numbers.values().cloned().collect();

    println!("oxygen: {:?}\nco2: {:?}", oxygen, co2);

    for i in 0..LINE_SIZE {
        let num_ones = oxygen.iter().map(|num| num[i]).sum::<u32>();
        let num_zeros = oxygen.len() as u32 - num_ones;
        let filter_num = if num_ones >= num_zeros { 1 } else { 0 };
        oxygen = oxygen.iter().filter(|l| l[i] == filter_num).cloned().collect();

        let num_ones = co2.iter().map(|num| num[i]).sum::<u32>();
        let num_zeros = co2.len() as u32 - num_ones;
        let filter_num = if num_zeros <= num_ones { 0 } else { 1 };
        co2 = co2.iter().filter(|l| l[i] == filter_num).cloned().collect();

        println!("{}\noxygen: \t{:?}\nco2: \t\t{:?}\n", i, oxygen, co2);
    }

    // oxygen: [0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1] = 931
    // co2: [1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0] = 3618
    // multiply: 3368358
}
