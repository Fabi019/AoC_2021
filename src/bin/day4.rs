#![feature(vec_retain_mut)]

use std::fmt::{Display, Formatter};

fn main() {
    let data = std::fs::read_to_string("assets/day4.txt").unwrap();
    let lines: Vec<&str> = data.split("\r\n\r\n").collect();

    let numbers: Vec<u8> = lines[0].split(',').map(|s| s.parse().expect(s)).collect();

    let mut boards = Vec::new();
    for board in 1..lines.len() {
        boards.push(BingoBoard::new(lines[board]));
    }

    for i in &numbers {
        // part one
        /*for board in &mut boards {
            if board.play(i) == true {
                println!("Board won:\n{}\nNumber: {}", board, i);
                return;
            }
        }*/

        // part two
        boards.retain_mut(|board| {
            let complete = board.play(i);
            if complete {
                println!("Board won:\n{}\nNumber: {}", board, i);
            }
            !complete
        })
    }
}

#[derive(Debug)]
struct BingoEntry {
    number: u8,
    marked: u8
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<BingoEntry>>
}

impl BingoBoard {
    pub fn new(content: &str) -> BingoBoard {
        let mut board = Vec::with_capacity(5);
        for line in content.lines() {
            board.push(
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .map(|number| BingoEntry { number, marked: 0 })
                    .collect::<Vec<BingoEntry>>(),
            );
        }
        BingoBoard { board }
    }

    pub fn play(&mut self, number: &u8) -> bool {
        self.mark(number);
        self.check()
    }

    fn mark(&mut self, number: &u8) {
        for vec in &mut self.board {
            for mut entry in vec {
                if &entry.number == number {
                    entry.marked = 1;
                }
            }
        }
    }

    fn check(&self) -> bool {
        self.check_rows() || self.check_columns()
    }

    fn check_rows(&self) -> bool {
        for vec in &self.board {
            let sum = vec.iter().map(|e| e.marked).sum::<u8>();
            if sum == 5 {
                return true
            }
        }
        false
    }

    fn check_columns(&self) -> bool {
        for column in 0..5 {
            let mut sum = 0;
            for row in 0..5 {
                sum += self.board[row][column].marked;
            }
            if sum == 5 {
                return true
            }
        }
        false
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sum_of_unmarked: u32 = 0;
        let mut output = String::new();
        for vec in &self.board {
            for entry in vec {
                if entry.marked == 1 {
                    output.push_str(&entry.number.to_string())
                } else {
                    sum_of_unmarked += entry.number as u32;
                }
                output.push_str("\t");
            }
            output.push_str("\n");
        }
        write!(f, "{}\nSum of unmarked: {}", output, sum_of_unmarked)
    }
}
