fn main() {
    let data = std::fs::read_to_string("assets/day6.txt").unwrap();
    let fish: Vec<u8> = data.split(',').map(|n| n.parse().unwrap()).collect();
    let mut fish_map = [0u64; 9];
    for &i in &fish {
        fish_map[i as usize] += 1;
    }

    const DAYS: u32 = 256;

    for current_day in 0..DAYS {
        let mut fish_to_add = 0;

        for age in 0..9 {
            let count = fish_map[age];
            if age == 0 {
                fish_to_add += count;
            } else {
                fish_map[age - 1] = count;
            }
            fish_map[age] = 0;
        }

        fish_map[8] += fish_to_add; // add all new fish
        fish_map[6] += fish_to_add; // convert 0 to 6 fish

        println!(
            "After {} day(s): -> {}",
            current_day + 1,
            fish_map.iter().sum::<u64>()
        );
    }
}
