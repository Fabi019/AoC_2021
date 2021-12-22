use std::cmp::{max, min};

fn main() {
    let data = std::fs::read_to_string("assets/day22.txt").unwrap();
    let cubes = data
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let state = split.next().unwrap() == "on";
            let split = split.next().unwrap().split(',');
            let mut xyz_range = split.map(|r| r.split('=').nth(1).unwrap()).map(|r| {
                let mut bounds = r.split("..");
                (
                    bounds.next().unwrap().parse::<i64>().unwrap(),
                    bounds.next().unwrap().parse::<i64>().unwrap(),
                )
            });
            Cuboid(
                xyz_range.next().unwrap(),
                xyz_range.next().unwrap(),
                xyz_range.next().unwrap(),
                state,
            )
        })
        //.filter(|c| c.is_small())
        .collect::<Vec<Cuboid>>();

    dbg!(&cubes);

    let mut done: Vec<Cuboid> = Vec::new();
    for cube in &cubes {
        let mut new = done.clone();
        for prev in done {
            if let Some(overlap) = prev.overlap(cube) {
                new.push(overlap);
            }
        }
        if cube.3 {
            new.push(*cube);
        }
        done = new;
    }

    let sum = done
        .iter()
        .map(|c| if c.3 { c.volume() } else { -c.volume() })
        .sum::<i64>();

    dbg!(sum);
}

#[derive(Debug, Copy, Clone)]
struct Cuboid((i64, i64), (i64, i64), (i64, i64), bool);

impl Cuboid {
    fn is_small(&self) -> bool {
        !(self.0 .0 < -50
            || self.1 .0 < -50
            || self.2 .0 < -50
            || self.0 .1 > 50
            || self.1 .1 > 50
            || self.2 .1 > 50)
    }
    fn volume(&self) -> i64 {
        (self.0 .1 - self.0 .0 + 1) * (self.1 .1 - self.1 .0 + 1) * (self.2 .1 - self.2 .0 + 1)
    }
    fn intersects(&self, other: &Self) -> bool {
        self.0 .0 <= other.0 .1
            && self.1 .0 <= other.1 .1
            && self.2 .0 <= other.2 .1
            && self.0 .1 >= other.0 .0
            && self.1 .1 >= other.1 .0
            && self.2 .1 >= other.2 .0
    }
    fn overlap(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        Some(Self(
            (max(self.0 .0, other.0 .0), min(self.0 .1, other.0 .1)),
            (max(self.1 .0, other.1 .0), min(self.1 .1, other.1 .1)),
            (max(self.2 .0, other.2 .0), min(self.2 .1, other.2 .1)),
            if self.3 != other.3 { other.3 } else { !other.3 },
        ))
    }
}
