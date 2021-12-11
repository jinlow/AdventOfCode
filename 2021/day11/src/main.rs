use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::str::FromStr;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BoxResult<()> {
    // let file = fs::read_to_string("small_input.txt").expect("File not found.");
    let file = fs::read_to_string("day11_input.txt").expect("File not found.");
    let lines: Vec<&str> = file.lines().collect();

    // Build data
    let mut data = Vec::new();
    let cols = lines[0].len();
    for l in lines {
        let recs = l
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        data.extend(recs);
    }

    let rows = data.len() / cols;

    let mut map = Matrix::from_vec(data.to_vec(), rows, cols);

    println!("{}", map);
    for _ in 0..100 {
        map.step(1, 0, 9);
    }

    println!("Total Flashes: {}", map.flashes);

    // Loop through all elements collecting the low points
    let mut map = Matrix::from_vec(data, rows, cols);
    for i in 0..400 {
        if map.step(1, 0, 9) {
            println!("Moment they all flashed: {}", i + 1);
            break;
        }
    }
    println!("Total Flashes: {}", map.flashes);
    Ok(())
}

// Simple contiguous matrix, that I made for the bingo problem.
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    stride1: usize,
    stride2: usize,
    flashes: u64,
}

impl<T> Matrix<T>
where
    T: FromStr
        + Clone
        + Copy
        + std::cmp::PartialOrd
        + std::fmt::Display
        + std::ops::Add<Output = T>,
    <T as FromStr>::Err: 'static + std::error::Error, // <T as FromStr>::Err: std::error::Error
{
    pub fn from_vec(data: Vec<T>, rows: usize, cols: usize) -> Self {
        Matrix {
            data,
            rows,
            cols,
            stride1: 1,
            stride2: cols,
            flashes: 0,
        }
    }

    pub fn step(&mut self, incrementer: T, min_value: T, max_value: T) -> bool {
        let mut flash_set: HashSet<(usize, usize)> = HashSet::new();
        self.increment(incrementer, max_value, &mut flash_set);
        // Now check for flashes
        for (i, j) in flash_set.clone() {
            self.flash(incrementer, min_value, max_value, &mut flash_set, i, j);
        }
        let all_flash = flash_set.len() == self.data.len();
        for (i, j) in flash_set {
            self.set(min_value, i, j);
        }
        return all_flash;
    }

    fn increment(&mut self, incrementer: T, max_value: T, flash_set: &mut HashSet<(usize, usize)>) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let v = *self.get(i, j);
                if v >= max_value {
                    flash_set.insert((i, j));
                } else {
                    self.set(v + incrementer, i, j);
                }
            }
        }
    }

    fn flash(
        &mut self,
        incrementer: T,
        min_value: T,
        max_value: T,
        flash_set: &mut HashSet<(usize, usize)>,
        i: usize,
        j: usize,
    ) {
        flash_set.insert((i, j));
        self.flashes += 1;
        for (ni, nj) in self.get_valid_points(i, j) {
            // If this is already a flash point, continue
            if flash_set.contains(&(ni, nj)) {
                continue;
            }
            // Otherwise, increment the value, or flash if it equals
            // The max value.
            let v = *self.get(ni, nj);
            if v >= max_value {
                self.flash(incrementer, min_value, max_value, flash_set, ni, nj);
            }
            self.set(v + incrementer, ni, nj)
        }
    }

    fn get_valid_points(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut v_i = vec![i];
        let mut v_j = vec![j];
        if i > 0 {
            v_i.push(i - 1);
        }
        if i < (self.rows - 1) {
            v_i.push(i + 1);
        }
        if j > 0 {
            v_j.push(j - 1);
        }
        if j < (self.cols - 1) {
            v_j.push(j + 1);
        }

        let mut points = Vec::new();
        for ni in v_i {
            for nj in &v_j {
                if (ni, nj) != (i, &j) {
                    points.push((ni, *nj));
                }
            }
        }
        points
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[self.item_index(i, j)]
    }

    pub fn set(&mut self, value: T, i: usize, j: usize) {
        let idx = self.item_index(i, j);
        self.data[idx] = value;
    }

    fn item_index(&self, i: usize, j: usize) -> usize {
        let mut idx: usize;
        idx = self.stride2 * i;
        idx = idx + (j * self.stride1);
        idx
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: FromStr + std::fmt::Display + Copy + std::cmp::PartialOrd + std::ops::Add<Output = T>,
    <T as FromStr>::Err: 'static + std::error::Error,
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut val = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                val.push_str(self.get(i, j).to_string().as_str());
                if j == (self.cols - 1) {
                    val.push('\n');
                } else {
                    val.push(' ');
                }
            }
        }
        write!(f, "{}", val)
    }
}
