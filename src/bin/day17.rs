use std::cmp::Ordering;

fn main() {
    let data = std::fs::read_to_string("assets/day17.txt")
        .unwrap()
        .replace("target area: ", "");
    let mut split = data
        .split(", ")
        .map(|s| s.split('=').nth(1).unwrap())
        .map(|s| {
            s.split("..")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });
    let x_box = split.next().unwrap();
    let y_box = split.next().unwrap();

    const MAX_STEPS: u32 = 300;
    const MAX_VEL: i32 = 300;

    let mut highest_valid_y = 0;
    let mut best_vel = (0, 0);
    let mut total_count = 0;

    let mut current_pos;
    let mut current_vel;
    let mut initial_vel;
    let mut max_y;
    for vel_y in -MAX_VEL..MAX_VEL {
        for vel_x in 1..MAX_VEL {
            max_y = 0;
            current_pos = (0, 0);
            current_vel = (vel_x, vel_y);
            initial_vel = current_vel;

            for step in 1..=MAX_STEPS {
                let (new_pos @ (x, y), new_vel) = next(current_pos, current_vel);
                if y > max_y {
                    max_y = y;
                }
                if x >= x_box[0] && x <= x_box[1] && y >= y_box[0] && y <= y_box[1] {
                    println!("Box reached after {} steps at pos {:?}", step, new_pos);
                    println!("Max height: {:?} with init vel: {:?}", max_y, initial_vel);
                    if max_y > highest_valid_y {
                        highest_valid_y = max_y;
                        best_vel = initial_vel;
                    }
                    total_count += 1;
                    break;
                } else if x > x_box[1] {
                    break;
                }
                current_pos = new_pos;
                current_vel = new_vel;
            }
        }
    }

    println!("Best run: {} with vel {:?}", highest_valid_y, best_vel);
    println!("Total valid runs: {}", total_count);
}

fn next(pos: (i32, i32), vel: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let vel_x = match vel.0.cmp(&0) {
        Ordering::Greater => vel.0 - 1,
        Ordering::Less => vel.0 + 1,
        _ => 0
    };
    ((pos.0 + vel.0, pos.1 + vel.1), (vel_x, vel.1 - 1))
}
