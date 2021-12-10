use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("assets/day10.txt").unwrap();

    let open_chars = "([{<";
    let closing_chars = ")]}>";
    let token_map: HashMap<char, char> = open_chars.chars().zip(closing_chars.chars()).collect();

    let point_map = HashMap::from([(')', 3u64), (']', 57), ('}', 1197), ('>', 25137)]);
    let score_map = HashMap::from([(')', 1u64), (']', 2), ('}', 3), ('>', 4)]);

    let mut total_points = 0;
    let mut scores = Vec::new();

    'outer: for line in data.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            if open_chars.contains(c) {
                stack.push(c);
            } else if closing_chars.contains(c) {
                let expected = token_map[&stack.pop().unwrap()];
                if c != expected {
                    println!("Expected '{}', but found '{}' instead.", expected, c);
                    total_points += point_map[&c];
                    continue 'outer;
                }
            }
        }
        scores.push(
            stack
                .iter()
                .rev()
                .fold(0, |acc, c| acc * 5 + score_map[&token_map[&c]]),
        );
    }
    scores.sort_unstable();
    let middle_score = scores[scores.len() / 2];

    println!("Middle score: {}", middle_score);
    println!("Total points: {}", total_points);
}
