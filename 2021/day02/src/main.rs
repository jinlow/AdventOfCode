use std::fs;

fn main() -> Result<(), std::io::Error> {
    let mut input: Vec<(String, i64)> = Vec::new();
    let file = fs::read_to_string("day02_input.txt")?;
    // file.lines()
    //     .map(|x| x.split(" ").parse::<i64>().unwrap())
    //     .for_each(|x| input.push(x));
    for line in file.lines() {
        let mut split = line.split(" ");
        input.push((split.next().unwrap().to_owned(), split.next().unwrap().parse::<i64>().unwrap()))
    }

    let mut s = Submarine::new();

    for m in &input {
        let (direction, distance) = m;
        s.move_submarine(direction.to_string(), *distance);
    };

    println!("The submarines position is {}", s.sub_dot());

    let mut aim_s = AimSubmarine::new();

    for m in &input {
        let (direction, distance) = m;
        aim_s.move_submarine(direction.to_string(), *distance);
    };

    println!("The aiming submarines position is {}", aim_s.sub_dot());

    Ok(())
}

struct AimSubmarine {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl AimSubmarine {
    fn new() -> Self {
        AimSubmarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn move_submarine(&mut self, direction: String, distance: i64) {
        if direction == "forward" {
            self.horizontal += distance;
            self.depth += self.aim * distance;
        }
        if direction == "down" {
            self.aim += distance;
        }
        if direction == "up" {
            self.aim -= distance;
        }
    }

    fn sub_dot(&self) -> i64 {
        self.depth * self.horizontal
    }
}

struct Submarine {
    horizontal: i64,
    depth: i64,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            horizontal: 0,
            depth: 0,
        }
    }

    fn move_submarine(&mut self, direction: String, distance: i64) {
        if direction == "forward" {
            self.horizontal += distance;
        }
        if direction == "down" {
            self.depth += distance;
        }
        if direction == "up" {
            self.depth -= distance;
        }
    }

    fn sub_dot(&self) -> i64 {
        self.depth * self.horizontal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_moving_sub() {
        let mut s = Submarine::new();
        let moves: Vec<(String, i64)> = vec![
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2),
        ];
        for m in moves {
            let (direction, distance) = m;
            s.move_submarine(direction, distance);
        };
        assert_eq!(s.sub_dot(), 150);
        
    }

    #[test]
    fn test_moving_aim_sub() {
        let mut s = AimSubmarine::new();
        let moves: Vec<(String, i64)> = vec![
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2),
        ];
        for m in moves {
            let (direction, distance) = m;
            s.move_submarine(direction, distance);
        };
        assert_eq!(s.sub_dot(), 900);
        
    }
}
