use std::collections::{BTreeMap, HashMap};

fn main() {
    let data = std::fs::read_to_string("assets/day6.txt").unwrap();
    let mut fishs: Vec<u8> = data.split(",").map(|n| n.parse().unwrap()).collect();
    let mut fish_map: BTreeMap<u8, u64> = BTreeMap::new();

    for &i in &fishs {
        *fish_map.entry(i).or_insert(0) += 1;
    }

    const DAYS: u32 = 256;

    for current_day in 0..DAYS {
        let mut fish_to_add = 0;

        for age in 0u8..9 {
            let mut count = *fish_map.entry(age).or_insert(0);
            if age == 0 {
                fish_to_add += count;
            } else {
                *fish_map.entry(age - 1).or_insert(0) = count;
            }
            *fish_map.entry(age).or_insert(0) = 0;
        }

        *fish_map.entry(8).or_insert(0) += fish_to_add; // add all new fish
        *fish_map.entry(6).or_insert(0) += fish_to_add; // convert 0 to 6 fish

        println!(
            "After {} day(s): -> {}",
            current_day + 1,
            fish_map.values().sum::<u64>()
        );
    }
}
