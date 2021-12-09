fn main() {
    let data = std::fs::read_to_string("assets/day9.txt").unwrap();

    let heightmap: Vec<Vec<u32>> = data
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut low_points = Vec::new();
    let mut total_risk = 0;
    for (y, vec) in heightmap.iter().enumerate() {
        for (x, height) in vec.iter().enumerate() {
            let neighbours = neighbours(&heightmap, x, y);
            let mut low_point = true;
            for (_, _, h) in &neighbours {
                if h <= height {
                    low_point = false;
                }
            }
            if low_point {
                total_risk += height + 1;
                low_points.push((x, y, height));
                println!("Low point found: {:?}", (x, y));
            }
        }
    }

    let mut basins: Vec<u32> = low_points
        .iter()
        .copied()
        .map(|(x, y, height)| {
            let mut done = vec![(x, y, *height)];
            let mut todo: Vec<(usize, usize, u32)> = neighbours(&heightmap, x, y);
            todo.retain(|e| e.2 != 9);
            println!("Starting for: {:?}", (x, y, height));
            println!("Initial: {:?}", todo);
            while !todo.is_empty() {
                for &e in &todo {
                    done.push(e);
                }
                todo = todo
                    .iter()
                    .flat_map(|e| neighbours(&heightmap, e.0, e.1))
                    .filter(|e| e.2 != 9 && !done.contains(e))
                    .collect();
                todo.sort_unstable();
                todo.dedup();
                println!("New: {:?}", todo);
            }
            done.len() as u32
        })
        .collect();
    basins.sort_unstable_by(|a, b| b.cmp(a)); // reverse order

    println!("Total risk: {}", total_risk);
    println!("Basins: {:?}", basins);
}

fn neighbours(vec: &[Vec<u32>], x: usize, y: usize) -> Vec<(usize, usize, u32)> {
    let mut neighbours = Vec::new();
    let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for offset in offsets {
        let target_x = (x as i32 + offset.0).clamp(0, (vec[0].len() - 1) as i32) as usize;
        let target_y = (y as i32 + offset.1).clamp(0, (vec.len() - 1) as i32) as usize;
        if target_y != y || target_x != x {
            neighbours.push((target_x, target_y, vec[target_y][target_x]));
        }
    }
    neighbours
}