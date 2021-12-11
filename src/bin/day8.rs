use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day8.txt").unwrap();
    let segments: Vec<SevenSegment> = data.lines().map(SevenSegment::new).collect();

    // Part one
    //let count_of_1_4_7_8 = segments.iter().flat_map(|s| s.target.clone()).map(|s| s.len()).filter(|&t| t == 2 || t == 4 || t == 3 || t == 7).count();
    //println!("{}", count_of_1_4_7_8);

    let mut total_num = 0;

    for seg in &segments {
        let mut number_map = HashMap::new();

        let one = find(&seg.segments, |s| s.len() == 2);
        let four = find(&seg.segments, |s| s.len() == 4);
        let seven = find(&seg.segments, |s| s.len() == 3);
        let eight = find(&seg.segments, |s| s.len() == 7);
        println!(
            "One = {:?}, Four = {:?}, Seven = {:?}, Eight = {:?}",
            one, four, seven, eight
        );

        let mut five_segments: Vec<Vec<char>> = seg
            .segments
            .iter()
            .filter(|s| s.len() == 5)
            .cloned()
            .collect(); // should be 2, 3, 5
        let three = find(&five_segments, |s| contains(s, &one));
        five_segments.retain(|s| s != &three);
        let five = find(&five_segments, |s| contains(s, &remove(&four, &one)));
        five_segments.retain(|s| s != &five);
        let two = five_segments.first().unwrap(); // last remaining
        println!("Three = {:?}, Five = {:?}, Two = {:?}", three, five, two);

        let mut six_segments: Vec<Vec<char>> = seg
            .segments
            .iter()
            .filter(|s| s.len() == 6)
            .cloned()
            .collect(); // should be 0, 6, 9
        let nine = find(&six_segments, |s| contains(s, &four));
        six_segments.retain(|s| s != &nine);
        let six = find(&six_segments, |s| contains(s, &five));
        six_segments.retain(|s| s != &six);
        let zero = six_segments.first().unwrap(); // last remaining
        println!("Nine = {:?}, Six = {:?}, Zero = {:?}", nine, six, zero);

        number_map.insert(zero, 0);
        number_map.insert(&one, 1);
        number_map.insert(two, 2);
        number_map.insert(&three, 3);
        number_map.insert(&four, 4);
        number_map.insert(&five, 5);
        number_map.insert(&six, 6);
        number_map.insert(&seven, 7);
        number_map.insert(&eight, 8);
        number_map.insert(&nine, 9);

        let mut number = 0;
        for (pos, value) in seg.target.iter().map(|e| number_map.get(e)).enumerate() {
            number += 10i32.pow((3 - pos) as u32) * value.unwrap_or(&0);
        }
        println!("Number: {}", number);
        total_num += number;
    }

    println!("Total: {}", total_num);
}

fn find<P>(vec: &[Vec<char>], predicate: P) -> Vec<char>
where
    P: FnMut(&Vec<char>) -> bool,
{
    vec.iter().cloned().find(predicate).unwrap()
}

fn contains(segment: &[char], chars: &[char]) -> bool {
    for c in chars {
        if !segment.contains(c) {
            return false;
        }
    }
    true
}

fn remove(chars: &[char], chars_to_remove: &[char]) -> Vec<char> {
    let mut rt = chars.to_vec();
    rt.retain(|c| !chars_to_remove.contains(c));
    rt
}

struct SevenSegment {
    segments: Vec<Vec<char>>,
    target: Vec<Vec<char>>,
}

impl SevenSegment {
    pub fn new(content: &str) -> SevenSegment {
        let parts: Vec<&str> = content.split(" | ").collect();
        let mut segments = parts[0]
            .split(' ')
            .map(|s| {
                let mut chars = s.chars().collect::<Vec<char>>();
                chars.sort_unstable();
                chars
            })
            .collect::<Vec<Vec<char>>>();
        segments.sort_unstable();
        let target = parts[1]
            .split(' ')
            .map(|s| {
                let mut chars = s.chars().collect::<Vec<char>>();
                chars.sort_unstable();
                chars
            })
            .collect::<Vec<Vec<char>>>();
        SevenSegment { segments, target }
    }
}
