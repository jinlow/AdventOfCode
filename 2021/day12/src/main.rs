use std::collections::{HashSet, VecDeque};

use std::fs;
fn main() {
    // let file = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    // let file = fs::read_to_string("small_input.txt").expect("Unable to find files");
    let file = fs::read_to_string("day12_input.txt").expect("Unable to find files");
    let all_links = file
        .lines()
        .map(|s| s.split("-").collect::<Vec<&str>>())
        .map(|p_vec| (p_vec[0], p_vec[1]))
        .collect::<Vec<(&str, &str)>>();

    // For each start, we will create a path
    let start_links: Vec<(&str, &str)> = all_links
        .iter()
        .filter(|(p1, p2)| p1 == &"start" || p2 == &"start")
        .map(|s| *s)
        .collect();

    let links: Vec<(&str, &str)> = all_links
        .iter()
        .filter(|(p1, p2)| p1 != &"start" && p2 != &"start")
        .map(|s| *s)
        .collect();

    // First we will create a path for each start
    let mut all_paths: VecDeque<FullPath> = VecDeque::new();
    for l in start_links.clone() {
        let path = FullPath::from_start(l, false);
        all_paths.push_front(path);
    }

    // Loop through the links over and over...
    // When a path ends, leave it popped off the all_paths
    // Que, and add it to the finished paths vector.
    let mut finished_paths: Vec<Vec<&str>> = Vec::new();
    while all_paths.len() > 0 {
        let p = all_paths.pop_back().unwrap();
        let first = *p.full_path.last().unwrap();
        if first == "end" {
            finished_paths.push(p.full_path);
        } else {
            for link in &links {
                if p.can_add_link(*link, first) {
                    let mut p_copy = p.clone();
                    p_copy.try_add_link(*link, first);
                    all_paths.push_front(p_copy);
                }
            }
        }
    }
    println!("Total Paths with 1 small cave : {}", finished_paths.len());

    // Part 2
    let mut all_paths: VecDeque<FullPath> = VecDeque::new();
    for l in start_links.clone() {
        let path = FullPath::from_start(l, true);
        all_paths.push_front(path);
    }
    let mut finished_paths: Vec<Vec<&str>> = Vec::new();
    while all_paths.len() > 0 {
        let p = all_paths.pop_back().unwrap();
        let first = *p.full_path.last().unwrap();
        if first == "end" {
            finished_paths.push(p.full_path);
        } else {
            for link in &links {
                let mut p_copy = p.clone();
                if p_copy.allow_two {
                    if p_copy.small_caves.contains(link.0) || p_copy.small_caves.contains(link.1) {
                        let mut link_set = HashSet::new();
                        link_set.insert(link.0);
                        link_set.insert(link.1);
                        p_copy.small_caves = p_copy
                            .small_caves
                            .difference(&link_set)
                            .map(|x| *x)
                            .collect::<HashSet<&str>>();
                        p_copy.allow_two = false;
                    }
                }
                if p_copy.can_add_link(*link, first) {
                    p_copy.try_add_link(*link, first);
                    all_paths.push_front(p_copy);
                }
            }
        }
    }
    println!("Total Paths with 2 small cave : {}", finished_paths.len());
}

#[derive(Debug, Clone, PartialEq)]
struct FullPath<'a> {
    full_path: Vec<&'a str>,
    small_caves: HashSet<&'a str>,
    allow_two: bool,
}

impl<'a> FullPath<'a> {
    fn can_add_link(&self, link: (&'a str, &'a str), first: &str) -> bool {
        if !self.old_small_cave(link) {
            if (link.0 == first) || (link.1 == first) {
                return true;
            }
        }
        false
    }
    fn try_add_link(&mut self, link: (&'a str, &'a str), first: &str) {
        // First make sure neither of the links are in the small
        // caves. This also will have the word "start" in it, this way
        // we never add another start link to the list.
        if !self.old_small_cave(link) {
            if link.0 == first {
                self.try_add_small_cave(link.0);
                self.full_path.push(link.1);
            } else if link.1 == first {
                self.try_add_small_cave(link.1);
                self.full_path.push(link.0);
            }
        }
    }
    fn old_small_cave(&self, link: (&str, &str)) -> bool {
        self.small_caves.contains(link.0) || self.small_caves.contains(link.1)
    }
    fn try_add_small_cave(&mut self, s: &'a str) {
        if s.to_lowercase().eq(s) {
            self.small_caves.insert(s);
        }
    }
    fn from_start(link: (&'a str, &'a str), allow_two: bool) -> Self {
        let mut full_path = Vec::new();
        let mut small_caves = HashSet::new();
        if link.0 == "start" {
            full_path.push(link.0);
            full_path.push(link.1);
        } else {
            full_path.push(link.1);
            full_path.push(link.0);
        }
        small_caves.insert("start");
        FullPath {
            full_path,
            small_caves,
            allow_two,
        }
    }
}
