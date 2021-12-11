fn main() {
    let data = std::fs::read_to_string("assets/day11.txt").unwrap();
    let mut octos: Vec<Vec<u32>> = data.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    println!("Berfore any steps: ");
    print(&octos);

    let mut total_flashed = 0;
    for step in 1.. {
        let flashed = make_step(&mut octos);
        println!("\nAfter step {}:", step);
        print(&octos);
        if flashed == 100 { // All octos flashed
            break;
        }
        total_flashed += flashed;
    }

    println!("Total flashed: {}", total_flashed);
}

fn print(octos: &[Vec<u32>]) {
    for line in octos {
        for octo in line {
            print!("{}", octo);
        }
        println!();
    }
}

fn make_step(octos: &mut [Vec<u32>]) -> usize {
    for octo in octos.iter_mut().flatten() {
        *octo += 1;
    }

    let mut total_flashed = Vec::new();
    loop {
        let mut flashed = Vec::new();

        for (y, vec) in octos.iter().enumerate() {
            for (x, octo) in vec.iter().enumerate() {
                if octo > &9 {
                    flashed.push((x, y));
                    total_flashed.push((x, y));
                }
            }
        }

        for (x, y) in &flashed {
            flash(octos, *x, *y);
        }

        if flashed.is_empty() {
            break;
        }
    }

    // Manually reset all flashed to 0
    for (x, y) in &total_flashed {
        octos[*y][*x] = 0;
    }

    total_flashed.len()
}

fn flash(octos: &mut [Vec<u32>], x: usize, y: usize) {
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            if x_off == 0 && y_off == 0 {
                continue;
            }

            let target_x = x as i32 + x_off;
            let target_y = y as i32 + y_off;

            if target_x < 0 || target_y < 0 || target_x >= octos[0].len() as i32 || target_y >= octos.len() as i32 {
                continue;
            }

            octos[target_y as usize][target_x as usize] += 1;
            octos[y][x] = u32::MIN;
        }
    }
}

