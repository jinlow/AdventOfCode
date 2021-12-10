use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    // let file = fs::read_to_string("small_input.txt").expect("File not Found.");
    let file = fs::read_to_string("day10_input.txt").expect("File not Found.");
    let lines: Vec<&str> = file.lines().collect();

    let checker = SyntaxChecker::new();

    let mut first_illegal = Vec::new();
    let mut missing_chars = Vec::new();

    for l in &lines {
        match checker.check_line(*l) {
            LineIssue::ExpectedFound(_, c2) => first_illegal.push(c2),
            LineIssue::Incomplete(v) => missing_chars.push(v),
            _ => continue,
        }
    }
    let mut corrupted_points_map: HashMap<char, u32> = HashMap::new();
    corrupted_points_map.insert(')', 3);
    corrupted_points_map.insert(']', 57);
    corrupted_points_map.insert('}', 1197);
    corrupted_points_map.insert('>', 25137);

    let mut total_corrupted_points = 0;
    for i in first_illegal {
        total_corrupted_points += corrupted_points_map.get(&i).unwrap();
    }

    println!("Total Corrupted Points {:?}", total_corrupted_points);

    // Part 2 Score
    let mut missing_points_map: HashMap<char, u64> = HashMap::new();
    missing_points_map.insert(')', 1);
    missing_points_map.insert(']', 2);
    missing_points_map.insert('}', 3);
    missing_points_map.insert('>', 4);

    let mut line_points = Vec::new();
    for v in missing_chars {
        let mut start = 0;
        for c in v {
            start = start * 5;
            start += missing_points_map.get(&c).unwrap();
        }
        line_points.push(start);
    }
    line_points.sort();
    let mid = line_points.len() / 2;
    let middle_missing_points: u64 = line_points[mid];
    println!("Total Corrupted Points {:?}", middle_missing_points);

    // // Example
    // let l = "{([(<{}[<>[]}>{[]{[(<()>";
    // let line_issue = checker.check_line(&l);
    // println!("{:?}", line_issue);

    // let l = "[({(<(())[]>[[{[]{<()<>>";
    // let line_issue = checker.check_line(&l);
    // println!("{:?}", line_issue);
}

#[derive(Debug)]
enum LineIssue {
    Incomplete(Vec<char>),
    ExpectedFound(char, char),
    LineOK,
}

struct SyntaxChecker {
    open_map: HashMap<char, char>,
    close_map: HashMap<char, char>,
    open_set: HashSet<char>,
    close_set: HashSet<char>,
}

impl SyntaxChecker {
    fn check_line(&self, line: &str) -> LineIssue {
        let mut unclosed_lines = VecDeque::new();
        for c in line.chars() {
            if self.open_set.contains(&c) {
                unclosed_lines.push_front(c);
            } else if self.close_set.contains(&c) {
                if let Some(start_c) = unclosed_lines.pop_front() {
                    // If the close set isn't in our map, we have a problem.
                    let open = self.close_map.get(&c).expect("Found unmapped character.");
                    if open != &start_c {
                        let expected = self
                            .open_map
                            .get(&start_c)
                            .expect("Found unmapped character.");
                        return LineIssue::ExpectedFound(expected.clone(), c.clone());
                    }
                }
            }
        }
        if unclosed_lines.len() > 0 {
            // Return the expected output
            let mut missing_chars = Vec::new();
            for c in unclosed_lines.iter() {
                missing_chars.push(self.open_map.get(c).unwrap().clone());
            }
            return LineIssue::Incomplete(missing_chars);
        }
        LineIssue::LineOK
    }

    fn new() -> Self {
        let open_sign = vec!['(', '[', '{', '<'];
        let close_sign = vec![')', ']', '}', '>'];
        let open_set: HashSet<char> = open_sign.iter().cloned().collect();
        let close_set: HashSet<char> = close_sign.iter().cloned().collect();
        let open_map: HashMap<char, char> = open_sign
            .iter()
            .zip(&close_sign)
            .map(|(o, c)| (*o, *c))
            .collect();
        let close_map: HashMap<char, char> = close_sign
            .iter()
            .zip(&open_sign)
            .map(|(c, o)| (*c, *o))
            .collect();

        SyntaxChecker {
            open_map,
            close_map,
            open_set,
            close_set,
        }
    }
}
