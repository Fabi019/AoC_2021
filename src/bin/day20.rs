use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day20.txt").unwrap();
    let mut lines = data.lines();

    let template = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == '#') as u32)
        .collect::<Vec<u32>>();
    let image = lines
        .skip(1)
        .map(|l| l.chars().map(|c| (c == '#') as u32).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let toggle_new = template[0];
    let height = image.len() as i32;
    let width = image[0].len() as i32;

    let mut pos_map = HashMap::new();
    for (y, vec) in image.into_iter().enumerate() {
        for (x, pixel) in vec.into_iter().enumerate() {
            pos_map.insert((x as i32, y as i32), pixel);
        }
    }

    for iteration in 1i32..=50 {
        let copy = pos_map.clone();
        let default = (1 - (iteration as u32 & 1)) & toggle_new;

        for y in 0 - iteration..=height + iteration {
            for x in 0 - iteration..=width + iteration {
                let value = get_value_at_pos(&copy, (x, y), default);
                let template_val = get_value_from_template(&template, value);
                pos_map.insert((x, y), template_val);
                //print!("{}", if template_val == 1 { "#" } else { "." });
            }
            //println!();
        }

        println!("Lit after {}: {}", iteration, pos_map.values().sum::<u32>());
    }

    println!("Total lit pixels: {}", pos_map.values().sum::<u32>());
}

fn get_value_from_template(template: &[u32], index: u32) -> u32 {
    template[index as usize]
}

fn get_value_at_pos(image: &HashMap<(i32, i32), u32>, pos: (i32, i32), default: u32) -> u32 {
    let mut sum = String::new();
    for y in -1..=1 {
        for x in -1..=1 {
            sum.push_str(
                &image
                    .get(&(pos.0 + x, pos.1 + y))
                    .unwrap_or(&default)
                    .to_string(),
            );
        }
    }
    u32::from_str_radix(&sum, 2).unwrap()
}
