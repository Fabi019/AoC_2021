use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day21.txt").unwrap();
    let mut positions = data
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut dice = (1..=100).cycle();
    let mut points = [0, 0];
    let mut throws = 0;

    'l: loop {
        for i in 0..positions.len() {
            let dice = (&mut dice).take(3).sum::<u32>();
            let new_pos = (positions[i] + dice - 1) % 10 + 1;
            positions[i] = new_pos;
            points[i] += new_pos;
            throws += 3;

            if points[i] >= 1000 {
                break 'l;
            }
        }
    }

    dbg!(points, throws);

    let (win1, win2) = roll_quantum_dice(9, 6, 0, 0, &mut HashMap::new());

    dbg!(win1, win2);
}

static OUTCOMES: [(u64, u64); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

fn roll_quantum_dice(
    pos1: u64,
    pos2: u64,
    score1: u64,
    score2: u64,
    universe: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) -> (u64, u64) {
    if score2 >= 21 {
        return (0, 1);
    }

    if let Some(&score) = universe.get(&(pos1, pos2, score1, score2)) {
        return score;
    }

    let (mut win1, mut win2) = (0, 0);
    for (occurrence, outcome) in OUTCOMES {
        let pos = (pos1 + outcome - 1) % 10 + 1;
        let score = score1 + pos;
        let (add2, add1) = roll_quantum_dice(pos2, pos, score2, score, universe);
        win1 += add1 * occurrence;
        win2 += add2 * occurrence;
    }

    universe.insert((pos1, pos2, score1, score2), (win1, win2));

    (win1, win2)
}
