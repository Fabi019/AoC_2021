#![feature(vec_retain_mut)]
#![feature(box_syntax)]
#![allow(dead_code)]

const BOARD_SIZE: usize = 1000;

fn main() {
    let data = std::fs::read_to_string("assets/day5.txt").unwrap();
    let mut lines: Vec<Line> = data
        .lines()
        .map(Line::new) /*.filter(Line::is_horizontal_or_vertical)*/
        .collect();

    let mut board = box [[0u32; BOARD_SIZE]; BOARD_SIZE];

    let mut two_or_more = 0;

    while !lines.is_empty() {
        lines.retain_mut(|l| {
            board[l.current.x as usize][l.current.y as usize] += 1;
            if board[l.current.x as usize][l.current.y as usize] == 2 {
                two_or_more += 1;
            }
            l.next()
        });
    }

    /*for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            print!("{}", board[x][y]);
        }
        println!();
    }*/

    println!("Fields with two or more: {}", two_or_more);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(point: &str) -> Point {
        let point: Vec<&str> = point.split(',').collect();
        Point {
            x: point[0].parse().unwrap(),
            y: point[1].parse().unwrap(),
        }
    }
}

struct Line {
    current: Point,
    end: Point,
}

impl Line {
    fn new(line: &str) -> Line {
        let points: Vec<&str> = line.split(" -> ").collect();
        Line {
            current: Point::new(points[0]),
            end: Point::new(points[1]),
        }
    }

    fn next(&mut self) -> bool {
        let x = self.current.x;
        let y = self.current.y;
        if x != self.end.x {
            let mut dx = self.end.x - x;
            dx /= i32::abs(dx); // normalise to +-1
            self.current.x += dx;
        }
        if y != self.end.y {
            let mut dy = self.end.y - y;
            dy /= i32::abs(dy); // normalise to +-1
            self.current.y += dy;
        }
        x != self.end.x || y != self.end.y
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.current.x == self.end.x || self.current.y == self.end.y
    }
}
