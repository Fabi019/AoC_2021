use std::collections::{HashMap, HashSet};

fn main() {
    let rotations = [
        (0, 1, 2, 1, 1, 1),
        (0, 2, 1, 1, 1, -1),
        (0, 1, 2, 1, -1, -1),
        (0, 2, 1, 1, -1, 1),
        (0, 1, 2, -1, 1, -1),
        (0, 2, 1, -1, 1, 1),
        (0, 1, 2, -1, -1, 1),
        (0, 2, 1, -1, -1, -1),
        (1, 0, 2, 1, 1, -1),
        (1, 2, 0, 1, -1, -1),
        (1, 0, 2, 1, -1, 1),
        (1, 2, 0, 1, 1, 1),
        (1, 0, 2, -1, 1, 1),
        (1, 2, 0, -1, 1, -1),
        (1, 0, 2, -1, -1, -1),
        (1, 2, 0, -1, -1, 1),
        (2, 1, 0, 1, 1, -1),
        (2, 0, 1, 1, -1, -1),
        (2, 1, 0, 1, -1, 1),
        (2, 0, 1, 1, 1, 1),
        (2, 1, 0, -1, 1, 1),
        (2, 0, 1, -1, 1, -1),
        (2, 1, 0, -1, -1, -1),
        (2, 0, 1, -1, -1, 1),
    ];

    let data = std::fs::read_to_string("assets/day19.txt").unwrap();
    let mut scanners = data
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    l.split(',')
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[i32; 3]>>()
        })
        .collect::<Vec<Vec<[i32; 3]>>>();

    let mut known_beacons = HashSet::from_iter(scanners[0].clone());
    let mut known_scanner = vec![[0, 0, 0]];

    scanners.remove(0); // remove scanner 0

    while !scanners.is_empty() {
        let mut progress = false;
        'outer: for (pos, scanner) in scanners.iter().enumerate() {
            for rotation in &rotations {
                let current = rotate(scanner, rotation);
                let (scanner_pos, overlaps) = check_overlap(&known_beacons, &current);
                if overlaps >= 12 {
                    println!("Overlaps detected with {}", pos);
                    let mut converted = Vec::new();
                    for pos in &current {
                        let target = [
                            pos[0] + scanner_pos[0],
                            pos[1] + scanner_pos[1],
                            pos[2] + scanner_pos[2],
                        ];
                        converted.push(target);
                    }
                    known_beacons.extend(converted);
                    known_scanner.push(scanner_pos);
                    scanners.remove(pos);
                    progress = true;
                    break 'outer;
                }
            }
        }
        assert!(progress);
    }

    let mut max_distance = 0;
    for a in &known_scanner {
        for b in &known_scanner {
            if a == b {
                continue;
            }
            let dst = i32::abs(a[0] - b[0]) + i32::abs(a[1] - b[1]) + i32::abs(a[2] - b[2]);
            if dst > max_distance {
                max_distance = dst;
            }
        }
    }

    dbg!(known_beacons.len());
    dbg!(max_distance);
}

fn rotate(vecs: &[[i32; 3]], rotation: &(i32, i32, i32, i32, i32, i32)) -> Vec<[i32; 3]> {
    vecs.iter()
        .copied()
        .map(|xyz| {
            [
                xyz[rotation.0 as usize] * rotation.3,
                xyz[rotation.1 as usize] * rotation.4,
                xyz[rotation.2 as usize] * rotation.5,
            ]
        })
        .collect::<Vec<[i32; 3]>>()
}

fn check_overlap(positions: &HashSet<[i32; 3]>, vecs: &[[i32; 3]]) -> ([i32; 3], i32) {
    let mut scanner_locs = HashMap::new();
    for pos in positions {
        for vec in vecs {
            let target = [pos[0] - vec[0], pos[1] - vec[1], pos[2] - vec[2]];
            *scanner_locs.entry(target).or_insert(0) += 1;
        }
    }
    scanner_locs.into_iter().max_by_key(|e| e.1).unwrap()
}
