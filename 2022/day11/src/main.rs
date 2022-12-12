use std::collections::VecDeque;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    inspections: usize,
    items: VecDeque<i64>,
    operation: String,
    test_div: i64,
    true_monkey: usize,
    false_monkey: usize,
}

fn split_to_int(s: &str, split: &str) -> i64 {
    s.split(split)
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .expect("unable to parse")
}

impl Monkey {
    fn from_monkey_string(s: String) -> Result<Self, Box<dyn Error>> {
        let lines = s.lines().collect::<Vec<&str>>();
        let items = lines[0]
            .split(": ")
            .nth(1)
            .expect("Unable to read items.")
            .split(", ")
            .map(|x| x.parse::<i64>().expect("Unable to parse items."))
            .collect::<VecDeque<i64>>();
        let operation = lines[1].split(" = ").nth(1).unwrap().to_string();
        let test_div = split_to_int(lines[2], " by ");
        let true_monkey = split_to_int(lines[3], "monkey ") as usize;
        let false_monkey = split_to_int(lines[4], "monkey ") as usize;
        Ok(Monkey {
            inspections: 0,
            items,
            operation,
            test_div,
            true_monkey,
            false_monkey,
        })
    }

    fn execute_operation(&mut self, old: i64) -> i64 {
        self.inspections += 1;
        let e = self.operation.replace("old", &old.to_string());
        if e.contains("+") {
            let vals = e
                .split(" + ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            vals[0] + vals[1]
        } else {
            let vals = e
                .split(" * ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            vals[0] * vals[1]
        }
    }

    fn test_item(&self, i: i64) -> usize {
        if i % self.test_div == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    // P1
    let mut monkeys = file
        .split("Monkey ")
        .filter(|x| x.len() > 0)
        // Drop the first line...
        .map(|s| s.lines().skip(1).collect::<Vec<&str>>().join("\n"))
        .map(|s| Monkey::from_monkey_string(s).unwrap())
        .collect::<Vec<Monkey>>();
    for _ in 0..20 {
        for m in 0..(monkeys.len()) {
            while !monkeys[m].items.is_empty() {
                if let Some(i) = monkeys[m].items.pop_front() {
                    let mut new_i = monkeys[m].execute_operation(i);
                    new_i = new_i / 3;
                    let test_item = monkeys[m].test_item(new_i);
                    monkeys[test_item].items.push_back(new_i);
                }
            }
        }
    }
    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    inspections.sort_by(|a, b| b.cmp(a));
    println!("{}", inspections[0] * inspections[1]);
    // P2
    let mut monkeys = file
        .split("Monkey ")
        .filter(|x| x.len() > 0)
        // Drop the first line...
        .map(|s| s.lines().skip(1).collect::<Vec<&str>>().join("\n"))
        .map(|s| Monkey::from_monkey_string(s).unwrap())
        .collect::<Vec<Monkey>>();
    let global_divisor = monkeys.iter_mut().map(|m| m.test_div).product::<i64>();

    for _ in 0..10000 {
        for m in 0..(monkeys.len()) {
            while !monkeys[m].items.is_empty() {
                if let Some(i) = monkeys[m].items.pop_front() {
                    let mut new_i = monkeys[m].execute_operation(i);
                    new_i = new_i % global_divisor;
                    let test_item = monkeys[m].test_item(new_i);
                    monkeys[test_item].items.push_back(new_i);
                }
            }
        }
    }
    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    inspections.sort_by(|a, b| b.cmp(a));
    println!("{}", inspections[0] * inspections[1]);
    Ok(())
}
