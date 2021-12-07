fn main() {
    let data = std::fs::read_to_string("assets/day7.txt").unwrap();
    let crabs: Vec<i32> = data.split(',').map(|n| n.parse().unwrap()).collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut lowest_fuel = i32::MAX;
    let mut best_target = 0;

    'outer: for target in min..max+1 {
        let mut total_fuel = 0;
        for &crab in &crabs {
            let mut fuel = i32::abs(crab - target);
            fuel = (fuel * (fuel + 1)) / 2; // part two
            total_fuel += fuel;
            if total_fuel > lowest_fuel {
                continue 'outer;
            }
        }
        if total_fuel < lowest_fuel {
            lowest_fuel = total_fuel;
            best_target = target;
        }
    }

    println!("Best total fuel {} at position {}", lowest_fuel, best_target);
}