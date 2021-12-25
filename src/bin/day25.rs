use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day25.txt").unwrap();

    let direction_map = HashMap::from([('.', (0, 0)), ('>', (1, 0)), ('v', (0, 1))]);
    let mut cucumbers = data
        .lines()
        .map(|l| l.chars().map(|c| direction_map[&c]).collect())
        .collect::<Vec<Vec<(i32, i32)>>>();

    for step in 1.. {
        let mut east_facing = Vec::new();
        let mut south_facing = Vec::new();

        for (y, vec) in cucumbers.iter().enumerate() {
            for (x, dir) in vec.iter().enumerate() {
                if dir == &(1, 0) {
                    east_facing.push((x, y));
                } else if dir == &(0, 1) {
                    south_facing.push((x, y));
                }
            }
        }

        let mut moved = false;

        let mut copy = cucumbers.clone();
        for (x, y) in east_facing {
            moved |= try_move(&cucumbers, &mut copy, (x, y), (1, 0));
        }
        cucumbers = copy;

        let mut copy = cucumbers.clone();
        for (x, y) in south_facing {
            moved |= try_move(&cucumbers, &mut copy, (x, y), (0, 1));
        }
        cucumbers = copy;

        if !moved {
            dbg!(step);
            break;
        }
    }
}

fn try_move(
    cucumbers: &[Vec<(i32, i32)>],
    insert: &mut [Vec<(i32, i32)>],
    at: (usize, usize),
    dir: (i32, i32),
) -> bool {
    let tx = (at.0 as i32 + dir.0) as usize % cucumbers[0].len();
    let ty = (at.1 as i32 + dir.1) as usize % cucumbers.len();
    if cucumbers[ty][tx] == (0, 0) {
        insert[ty][tx] = dir;
        insert[at.1][at.0] = (0, 0);
        true
    } else {
        false
    }
}
