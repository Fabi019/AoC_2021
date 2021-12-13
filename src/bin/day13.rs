fn main() {
    let data = std::fs::read_to_string("assets/day13.txt").unwrap();
    let mut paper: Vec<(u32, u32)> = data
        .lines()
        .filter(|l| !l.starts_with("fold") && !l.is_empty())
        .map(|l| {
            let mut split = l.split(',');
            let x: u32 = split.next().unwrap().parse().unwrap();
            let y: u32 = split.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let instructions: Vec<(String, u32)> = data
        .lines()
        .filter(|l| l.starts_with("fold"))
        .map(|i| {
            let modified = i.replace("fold along ", "");
            let mut split = modified.split('=');
            let axis = split.next().unwrap();
            let val: u32 = split.next().unwrap().parse().unwrap();
            (axis.to_string(), val)
        })
        .collect();

    println!("{:?}", paper);
    println!("{:?}", instructions);

    for ins in &instructions {
        println!("Folding {:?}", ins);
        if ins.0 == "x" {
            paper
                .iter_mut()
                .filter(|e| e.0 > ins.1)
                .for_each(|e| e.0 = ins.1 - (e.0 - ins.1));
        } else {
            paper
                .iter_mut()
                .filter(|e| e.1 > ins.1)
                .for_each(|e| e.1 = ins.1 - (e.1 - ins.1));
        }

        paper.sort_unstable();
        paper.dedup();

        println!("Result: {:?}", paper);
    }

    let maxx = paper.iter().max_by_key(|e| e.0).unwrap().0;
    let maxy = paper.iter().max_by_key(|e| e.1).unwrap().1;

    for y in 0u32..=maxy {
        for x in 0u32..=maxx {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("Final count: {}", paper.len());
}
