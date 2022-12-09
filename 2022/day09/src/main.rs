use std::collections::HashSet;
use std::error::Error;
use std::fs;

enum Movement {
    R(i64),
    L(i64),
    U(i64),
    D(i64),
}

fn parse_movement(s: &str) -> Result<Movement, Box<dyn Error>> {
    let m = s.split(" ").collect::<Vec<&str>>();
    let steps = m[1].parse::<i64>()?;
    match m[0] {
        "R" => Ok(Movement::R(steps)),
        "L" => Ok(Movement::L(steps)),
        "U" => Ok(Movement::U(steps)),
        "D" => Ok(Movement::D(steps)),
        _ => Err("Unparsable".into()),
    }
}

struct Rope {
    knots: Vec<(i64, i64)>,
    tail_points: HashSet<(i64, i64)>,
}

impl Rope {
    fn new(n_knots: usize) -> Self {
        let mut knots = Vec::new();
        for _ in 0..n_knots {
            knots.push((0, 0));
        }
        let mut rope = Rope {
            knots,
            tail_points: HashSet::new(),
        };
        rope.tail_points.insert((0, 0));
        rope
    }

    fn move_steps(&mut self, steps: i64, vertical: i64, horizontal: i64) {
        for _ in 0..steps {
            // Move the head once...
            self.knots[0].0 += vertical.signum();
            self.knots[0].1 += horizontal.signum();
            // Now adjust all the other knots...
            // Starting with the head...
            for i in 0..(self.knots.len() - 1) {
                self.adjust_knot(i, i + 1)
            }
            self.tail_points.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn move_rope(&mut self, movement: Movement) {
        match movement {
            Movement::R(n) => self.move_steps(n, 0, 1),
            Movement::L(n) => self.move_steps(n, 0, -1),
            Movement::U(n) => self.move_steps(n, 1, 0),
            Movement::D(n) => self.move_steps(n, -1, 0),
        }
    }

    fn adjust_knot(&mut self, first: usize, second: usize) {
        let k1 = self.knots[first];
        let k2 = &mut self.knots[second];
        let vertical_diff = k1.0 - k2.0;
        let horizontal_diff = k1.1 - k2.1;
        // Don't need to move at all...
        if (vertical_diff.abs() <= 1) && (horizontal_diff.abs() <= 1) {
            return;
        // Move straight up or down...
        } else if (vertical_diff.abs() > 1) && (horizontal_diff == 0) {
            k2.0 += vertical_diff.signum();
        // Otherwise move left or right...
        } else if (vertical_diff == 0) && (horizontal_diff.abs() > 1) {
            k2.1 += horizontal_diff.signum();
        // Finally move diagonally...
        } else {
            k2.0 += vertical_diff.signum();
            k2.1 += horizontal_diff.signum();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let file = fs::read_to_string("input.txt")?;
    let mut rope = Rope::new(2);
    for l in file.lines() {
        let m = parse_movement(l)?;
        rope.move_rope(m)
    }
    println!("{}", rope.tail_points.len());

    // P2
    let mut rope = Rope::new(10);
    for l in file.lines() {
        let m = parse_movement(l)?;
        rope.move_rope(m)
    }
    println!("{}", rope.tail_points.len());


    Ok(())
}
