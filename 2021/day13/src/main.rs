use std::collections::HashSet;
use std::fmt;
use std::{fs, num::ParseIntError};

type BoxError<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BoxError<()> {
    // let file = fs::read_to_string("small_input.txt").expect("File not found");
    let file = fs::read_to_string("day13_input.txt").expect("File not found");
    let mut points = Vec::new();

    let mut fold_along: Vec<(&str, u64)> = Vec::new();

    // Gather data
    for (i, l) in file.lines().enumerate() {
        if l == "" {
            let folds = file.lines().collect::<Vec<&str>>()[(i + 1)..].to_vec();
            for f in folds {
                let s = f.split("along ").collect::<Vec<&str>>()[1];
                let sv = s.split("=").collect::<Vec<&str>>();
                fold_along.push((sv[0], sv[1].parse::<u64>()?));
            }
            break;
        }
        let lv = l
            .split(",")
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;
        points.push((lv[0], lv[1]));
    }

    points.sort();

    let mut folded_points = points
        .iter()
        .map(|p| fold_point(p, fold_along[0].0, fold_along[0].1))
        .collect::<HashSet<(u64, u64)>>();

    // Part 1
    println!("N points after first fold: {}", folded_points.len());

    // Now continue the folds
    for f in fold_along[1..].to_vec() {
        folded_points = folded_points
            .iter()
            .map(|p| fold_point(p, f.0, f.1))
            .collect::<HashSet<(u64, u64)>>();
    }
    println!("N points after all folds: {}", folded_points.len());

    let fold_vec = folded_points
        .iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>();

    let max_x = match fold_vec.iter().map(|(x, _)| x).max() {
        Some(v) => *v,
        None => 0,
    };
    let max_y = match fold_vec.iter().map(|(_, y)| y).max() {
        Some(v) => *v,
        None => 0,
    };
    println!("Max X: {}, and Y: {}", max_x, max_y);

    let mut sign = Matrix::from_constant(".", max_y + 1, max_x + 1);
    for (i, j) in fold_vec {
        sign.set("#", j, i);
    }
    println!("{}", sign);

    Ok(())
}

fn fold_point(point: &(u64, u64), axis: &str, fold: u64) -> (u64, u64) {
    if axis == "x" {
        if point.0 >= fold {
            (point.0 - ((point.0 - fold) * 2), point.1)
        } else {
            *point
        }
    } else {
        if point.1 >= fold {
            (point.0, point.1 - ((point.1 - fold) * 2))
        } else {
            *point
        }
    }
}

struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    stride1: usize,
    stride2: usize,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn from_constant(value: T, rows: usize, cols: usize) -> Self {
        let data = vec![value; rows * cols];
        Matrix {
            data,
            rows,
            cols,
            stride1: 1,
            stride2: cols,
        }
    }

    fn item_index(&self, i: usize, j: usize) -> usize {
        let mut idx: usize;
        idx = self.stride2 * i;
        idx = idx + (j * self.stride1);
        idx
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[self.item_index(i, j)]
    }

    pub fn set(&mut self, value: T, i: usize, j: usize) {
        let idx = self.item_index(i, j);
        self.data[idx] = value;
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: std::fmt::Display + Copy,
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
