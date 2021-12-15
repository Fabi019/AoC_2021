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
    let total_risk = dijkstra(&cave, (0, 0), (499, 499)).unwrap();

    println!("Total risk: {}", total_risk);
}

fn dijkstra(
    cave: &[Vec<u32>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut dist = HashMap::from([(start, 0)]);
    let mut heap = BinaryHeap::from(vec![Reverse((0, start))]);

    while let Some(Reverse((total_risk, pos))) = heap.pop() {
        if pos == end {
            return Some(total_risk);
        }

        if total_risk > dist[&pos] {
            continue;
        }

        for (risk, nb_pos) in neighbours(cave, pos) {
            let next @ (new_risk, _) = (total_risk + risk, nb_pos);
            if new_risk < *dist.entry(nb_pos).or_insert(u32::MAX) {
                heap.push(Reverse(next));
                dist.insert(nb_pos, new_risk);
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
        let part_x = target_x % cave[0].len();
        risk_inc += (target_x - part_x) / cave[0].len();
        let part_y = target_y % cave.len();
        risk_inc += (target_y - part_y) / cave.len();

        let mut risk = cave[part_y][part_x] + risk_inc as u32;
        if risk > 9 {
            risk = risk % 10 + 1;
        }

        if target_y != pos.1 || target_x != pos.0 {
            neighbours.push((risk, (target_x, target_y)));
        }
    }
    neighbours
}
