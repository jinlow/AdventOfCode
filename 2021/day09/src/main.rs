use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::str::FromStr;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BoxResult<()> {
    // let file = fs::read_to_string("small_input.txt").expect("File not found.");
    let file = fs::read_to_string("day09_input.txt").expect("File not found.");
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

    let map = Matrix::from_vec(data, rows, cols);

    // Loop through all elements collecting the low points
    let mut risk_levels = Vec::new();
    let mut low_points = Vec::new();
    for i in 0..map.rows {
        for j in 0..map.cols {
            if let Some(e) = map.low_element(i, j) {
                risk_levels.push(e + 1);
                low_points.push((i, j));
            }
        }
    }

    let total_risk: u64 = risk_levels.iter().map(|r| *r as u64).sum();

    println!("Total Risk: {}", total_risk);

    // Part two
    let mut sizes = map.basin_sizes(&low_points, &9);

    sizes.sort_by(|a, b| b.cmp(a));
    let top_three = sizes[0..3].to_vec();
    println!("{:?}", top_three);

    let top_three_sum: u64 = top_three.iter().product();
    println!("Product of the top three Basins {}", top_three_sum);

    Ok(())
}

// Simple contiguous matrix, that I made for the bingo problem.
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    stride1: usize,
    stride2: usize,
}

impl<T> Matrix<T>
where
    T: FromStr + Clone + std::cmp::PartialOrd + std::fmt::Display,
    <T as FromStr>::Err: 'static + std::error::Error, // <T as FromStr>::Err: std::error::Error
{
    pub fn from_vec(data: Vec<T>, rows: usize, cols: usize) -> Self {
        Matrix {
            data,
            rows,
            cols,
            stride1: 1,
            stride2: cols,
        }
    }

    pub fn basin_sizes(&self, low_points: &[(usize, usize)], max_element: &T) -> Vec<u64> {
        let mut sizes = Vec::new();
        for (i, j) in low_points {
            let mut checked_points = HashSet::new();
            let basin_size =
                self.basin_size_recursive(*i, *j, None, None, max_element, &mut checked_points);
            sizes.push(basin_size);
        }
        sizes
    }

    fn basin_size_recursive(
        &self,
        i: usize,
        j: usize,
        start_i: Option<usize>,
        start_j: Option<usize>,
        max_element: &T,
        checked_points: &mut HashSet<(usize, usize)>,
    ) -> u64 {
        let element = self.get(i, j);
        let mut search_points = Vec::new();
        if i > 0 {
            search_points.push((i - 1, j));
        }
        search_points.push((i + 1, j));
        if j > 0 {
            search_points.push((i, j - 1));
        }
        search_points.push((i, j + 1));

        // println!("{:?}", search_points);

        let mut check_points = Vec::new();
        for (i_s, j_s) in search_points {
            // If this is the point we came from, skip it.
            if !start_i.is_none() && !start_j.is_none() {
                if (i_s == start_i.unwrap()) && (j_s == start_j.unwrap()) {
                    continue;
                }
            }
            if checked_points.contains(&(i_s, j_s)) {
                continue;
            }

            if (i_s < self.rows) && (j_s < self.cols) {
                let check_element = self.get(i_s, j_s);
                if (check_element < max_element) && (check_element > element) {
                    // println!("From: {}, {:?}, to: {}, {:?}", element, (i, j), check_element, (i_s, j_s));
                    check_points.push((i_s, j_s));
                    checked_points.insert((i_s, j_s));
                }
            }
        }
        // return 1 + self.basin_size_recursive(i_s, j_s, Some(i), Some(j), max_element)
        return 1 + check_points
            .iter()
            .map(|(i_s, j_s)| {
                self.basin_size_recursive(*i_s, *j_s, Some(i), Some(j), max_element, checked_points)
            })
            .sum::<u64>();
    }

    pub fn low_element(&self, i: usize, j: usize) -> Option<&T> {
        let element = self.get(i, j);
        // If possible check above
        if i > 0 {
            if self.get(i - 1, j) <= element {
                return None;
            }
        }
        // If possible check left
        if j > 0 {
            if self.get(i, j - 1) <= element {
                return None;
            }
        }
        // If possible check to the bellow
        if i < (self.rows - 1) {
            if self.get(i + 1, j) <= element {
                return None;
            }
        }
        // If possible check right
        if j < (self.cols - 1) {
            if self.get(i, j + 1) <= element {
                return None;
            }
        }
        return Some(element);
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
    T: FromStr + std::fmt::Display + Copy + std::cmp::PartialOrd,
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
