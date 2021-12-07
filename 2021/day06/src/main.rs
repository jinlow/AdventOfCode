use std::{fs, num::ParseIntError};
fn main() -> BoxResult<()> {
    // let file = fs::read_to_string("small_input.txt").expect("File not found.");
    let file = fs::read_to_string("day06_input.txt").expect("File not found.");
    let initial_times = file.split(",").map(|f| f.parse::<usize>()).collect::<Result<Vec<usize>, ParseIntError>>()?;
    let mut fishcounter = FishCounter::new();

    // Part 1
    fishcounter.add_fish_from_days(&initial_times);
    for _ in 0..80 {
        fishcounter.decrement_spawn_counter();
    }
    println!("Total number of fish after 80 {}", fishcounter.fishcnt.iter().sum::<u64>());

    // Part 2
    let mut fishcounter = FishCounter::new();
    fishcounter.add_fish_from_days(&initial_times);
    for _ in 0..256 {
        fishcounter.decrement_spawn_counter();
    }
    println!("Total number of fish after 256 {}", fishcounter.fishcnt.iter().sum::<u64>());
    Ok(())
}

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

struct FishCounter {
    fishcnt: Vec<u64>,
}

impl FishCounter {
    fn new() -> FishCounter {
        FishCounter {
            fishcnt: vec![0;9],

        }
    }
    fn add_fish_from_days(&mut self, fish: &[usize]) {
        for f in fish {
            self.fishcnt[*f] += 1;
        }
    }
    fn decrement_spawn_counter(&mut self) {
        let zero_cnt = self.fishcnt[0];
        self.fishcnt.rotate_left(1);
        self.fishcnt[6] += zero_cnt;
    }
}
