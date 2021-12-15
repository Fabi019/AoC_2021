use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day15.txt").unwrap();

    let cave = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let (_path, total_risk) = dijkstra(&cave, (0, 0), (499, 499)).unwrap();

    println!("Total risk: {}", total_risk);
}

fn dijkstra(
    cave: &[Vec<u32>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(Vec<(usize, usize)>, u32)> {
    let mut dist = HashMap::new();
    // prev is not needed at all in this task since only the total risk of the path needs to be known
    let mut prev = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((total_risk, pos))) = heap.pop() {
        if pos == end {
            let mut path = Vec::new();
            let mut current = end;
            while let Some(next) = prev.get(&current) {
                path.push(*next);
                current = *next;
            }
            path.reverse();
            return Some((path, total_risk));
        }

        if total_risk > *dist.entry(pos).or_insert(u32::MAX) {
            continue;
        }

        for (risk, nb_pos @ (x, y)) in neighbours(cave, pos) {
            let next = (total_risk + risk, nb_pos);
            if next.0 < *dist.entry((x, y)).or_insert(u32::MAX) {
                heap.push(Reverse(next));
                dist.insert(nb_pos, next.0);
                prev.insert(nb_pos, pos);
            }
        }
    }
    None
}

fn neighbours(cave: &[Vec<u32>], pos: (usize, usize)) -> Vec<(u32, (usize, usize))> {
    let mut neighbours = Vec::new();
    let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (x, y) in offsets {
        let target_x = (pos.0 as i32 + x).clamp(0, (cave[0].len() * 5) as i32 - 1) as usize;
        let target_y = (pos.1 as i32 + y).clamp(0, (cave.len() * 5) as i32 - 1) as usize;

        let mut risk_inc = 0;

        // Start part two
        if target_x >= cave[0].len() {
            risk_inc += (target_x - target_x % cave[0].len()) / cave[0].len();
        }
        if target_y >= cave.len() {
            risk_inc += (target_y - target_y % cave.len()) / cave.len();
        }
        // End Part two

        let mut risk = cave[target_y % cave.len()][target_x % cave[0].len()] + risk_inc as u32;
        if risk > 9 {
            risk = risk % 10 + 1;
        }

        if target_y != pos.1 || target_x != pos.0 {
            neighbours.push((risk, (target_x, target_y)));
        }
    }
    neighbours
}
