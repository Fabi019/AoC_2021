use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day14.txt").unwrap();

    let template = data.lines().next().unwrap().chars().collect::<Vec<char>>();
    let instructions = data
        .lines()
        .skip(2)
        .map(|l| {
            let mut split = l.split(" -> ");
            let pat = split.next().unwrap().chars().collect::<Vec<char>>();
            let ins = split.next().unwrap().chars().next().unwrap();
            ((pat[0], pat[1]), ins)
        })
        .collect::<HashMap<(char, char), char>>();

    let mut template_map = HashMap::new();
    for pattern in template.windows(2) {
        *template_map.entry((pattern[0], pattern[1])).or_insert(0u64) += 1;
    }

    for _ in 1..=40 {
        for (pat, count) in template_map.clone() {
            if let Some(ins) = instructions.get(&pat) {
                *template_map.entry(pat).or_insert(0) -= count;
                *template_map.entry((pat.0, *ins)).or_insert(0) += count;
                *template_map.entry((*ins, pat.1)).or_insert(0) += count;
            }
        }
    }

    let letter_counts: HashMap<char, (u64, u64)> =
        template_map.into_iter().fold(HashMap::new(), |mut map, ((a, b), count)| {
            map.entry(a).or_insert((0, 0)).0 += count;
            map.entry(b).or_insert((0, 0)).1 += count;
            map
        });

    let counts = letter_counts
        .values()
        .copied()
        .map(|(a, b)| a.max(b))
        .collect::<Vec<u64>>();

    let max = counts.iter().max().unwrap();
    let min = counts.iter().min().unwrap();

    println!(
        "{:?}\n{:?}\n{:?} - {:?} = {}",
        letter_counts,
        counts,
        max,
        min,
        max - min
    );
}
