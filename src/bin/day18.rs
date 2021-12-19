#![feature(box_syntax)]

use std::str::Chars;

use crate::Element::{Number, Pair};

fn main() {
    let data = std::fs::read_to_string("assets/day18.txt").unwrap();

    let snail_nums = data
        .lines()
        .map(|l| parse_seq(&mut l.chars()))
        .collect::<Vec<Element>>();

    // Convert first element to snailfish number
    let mut sum = snail_nums[0].to_snail();

    println!("Initial num: {:?}", sum);

    for snail_num in snail_nums.clone().into_iter().skip(1) {
        sum = sum.add(snail_num);
        //println!("After add: {:?}", sum);
        sum.reduce();
    }

    dbg!(sum.magnitude());

    let mut highest_sum = 0;
    for first_num in snail_nums.clone().into_iter() {
        for second_num in snail_nums.clone().into_iter() {
            if first_num == second_num {
                continue;
            }

            let mut sum = first_num.to_snail().add(second_num);
            sum.reduce();
            let sum = sum.magnitude();
            if sum > highest_sum {
                highest_sum = sum;
            }
        }
    }

    dbg!(highest_sum);
}

fn parse_seq(seq: &mut Chars) -> Element {
    let c = seq.next().unwrap();
    match c {
        '[' => {
            let x = parse_seq(seq);
            seq.next(); // consume ","
            let y = parse_seq(seq);
            seq.next(); // consume "]"
            Pair(box x, box y)
        }
        _ => Number(c.to_digit(10).unwrap().try_into().unwrap()),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct SnailNumber(Element, Element);

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Number(u8),
    Pair(Box<Element>, Box<Element>),
}

impl SnailNumber {
    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }
    fn explode(&mut self) -> bool {
        self.0.explode(1, None, Some(self.1.find(-1)))
            || self.1.explode(1, Some(self.0.find(1)), None)
    }
    fn split(&mut self) -> bool {
        self.0.split() || self.1.split()
    }
    fn add(self, second: Element) -> Self {
        SnailNumber(Pair(box self.0, box self.1), second)
    }
    fn magnitude(&self) -> usize {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

impl Element {
    fn to_snail(&self) -> SnailNumber {
        if let Pair(x, y) = self.clone() {
            SnailNumber(*x, *y)
        } else {
            panic!("Element is not a Pair!")
        }
    }
    fn find(&mut self, dir: i8) -> &mut u8 {
        match self {
            Number(i) => i,
            Pair(x, y) => if dir == 1 { y } else { x }.find(dir),
        }
    }
    fn explode(
        &mut self,
        current_level: usize,
        last_x: Option<&mut u8>,
        last_y: Option<&mut u8>,
    ) -> bool {
        match self {
            Number(_) => false,
            Pair(x, y) => {
                if current_level == 4 {
                    if let Number(x) = **x {
                        if let Some(last) = last_x {
                            *last += x;
                        }
                    }
                    if let Number(y) = **y {
                        if let Some(last) = last_y {
                            *last += y;
                        }
                    }
                    *self = Number(0);
                    true
                } else {
                    x.explode(current_level + 1, last_x, Some(y.find(-1)))
                        || y.explode(current_level + 1, Some(x.find(1)), last_y)
                }
            }
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Number(i) => {
                if *i >= 10 {
                    let x = Number(*i / 2);
                    let y = Number(*i / 2 + *i % 2);
                    *self = Pair(box x, box y);
                    true
                } else {
                    false
                }
            }
            Pair(x, y) => x.split() || y.split(),
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            Number(i) => *i as usize,
            Pair(x, y) => 3 * x.magnitude() + 2 * y.magnitude(),
        }
    }
}
