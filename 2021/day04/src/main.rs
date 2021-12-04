use regex::Regex;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
fn main() -> BoxResult<()> {
    let v = vec![1, 2, 3, 4, 5, 6];
    let b = Matrix::from_vec(v, 3, 2);
    println!("{}", b);

    // let file = fs::read_to_string("small_input.txt").expect("Unable to read file.");
    let file = fs::read_to_string("day04_input.txt").expect("Unable to read file.");
    let lines = Regex::new(r"\r\n\r\n|\n\n")?
        .split(&file)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let str_moves = &lines[0];
    let moves: Vec<i64> = str_moves
        .split(",")
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    println!("{:?}\n", moves);
    let mut bingo = BingoBoards::from_strings(&lines[1..])?;
    println!("We collected {} boards.", bingo.boards.len());
    for b in &bingo.boards {
        println!("{}", b);
    }

    // Part 1
    for m in &moves {
        if bingo.check_boards(*m) {
            break;
        }
    }
    let (unplayed, winning, product) = bingo.winning_stats();
    println!("For the first winning board.");
    println!(
        "Total of unplayed: {}, Winning Number: {}, Product: {}",
        unplayed, winning, product
    );

    // Part 2
    // Now we want to know the last board to win, let's keep a set of the winning board numbers.
    // Once that set is the same length as the number of boards, we have our answer.
    let mut bingo = BingoBoards::from_strings(&lines[1..])?;
    let n_boards = &bingo.boards.len();
    let mut winning_boards = HashSet::new();
    for m in &moves {
        if bingo.check_boards(*m) {
            winning_boards.insert(bingo.winning_idx.unwrap());
            if winning_boards.len() == *n_boards {
                break;
            }
        }
    }

    let (unplayed, winning, product) = bingo.winning_stats();
    println!("For the last winning board.");
    println!(
        "Total of unplayed: {}, Winning Number: {}, Product: {}",
        unplayed, winning, product
    );

    Ok(())
}

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

struct BingoBoards {
    boards: Vec<Matrix<i64>>,
    played: Vec<Matrix<u8>>,
    won: Vec<bool>,
    winning_idx: Option<usize>,
    winning_number: Option<i64>,
}

impl BingoBoards {
    pub fn from_strings(lines: &[String]) -> BoxResult<Self> {
        let mut boards: Vec<Matrix<i64>> = Vec::new();
        for block in lines {
            boards.push(Matrix::from_lines(
                block
                    .lines()
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<String>>(),
            )?)
        }
        let played = boards
            .iter()
            .map(|b| Matrix::from_constant(0, b.rows, b.cols))
            .collect();
        let won = vec![false; boards.len()];
        Ok(BingoBoards {
            boards,
            played,
            won,
            winning_idx: None,
            winning_number: None,
        })
    }
    pub fn check_boards(&mut self, value: i64) -> bool {
        let mut board_won = false;
        for (board_idx, (b, p)) in self.boards.iter().zip(&mut self.played).enumerate() {
            if self.won[board_idx] {
                continue;
            }
            for i in 0..b.rows {
                for j in 0..b.cols {
                    if b.get(i, j) == &value {
                        p.set(1, i, j);
                        // Now check if they have won..
                        // We only need to check the intersection
                        // of this i and j.
                        let mut row_total = 0;
                        for row in 0..p.cols {
                            row_total += p.get(row, j);
                        }
                        if (row_total as usize) == p.rows {
                            println!("We won with number {} on board {}!", value, board_idx);
                            self.winning_idx = Some(board_idx);
                            self.winning_number = Some(value);
                            self.won[board_idx] = true;
                            board_won = true;
                        }
                        let mut col_total = 0;
                        for col in 0..p.rows {
                            col_total += p.get(i, col);
                        }
                        if (col_total as usize) == p.cols {
                            println!("We won with number {} on board {}!", value, board_idx);
                            self.winning_idx = Some(board_idx);
                            self.winning_number = Some(value);
                            self.won[board_idx] = true;
                            board_won = true;
                        }
                    }
                }
            }
        }
        board_won
    }
    pub fn winning_stats(&self) -> (i64, i64, i64) {
        let win_idx = self.winning_idx.unwrap();
        let board_nums = &self.boards[win_idx].data;
        let played_nums = &self.played[win_idx].data;
        let total_unplayed: i64 = board_nums
            .iter()
            .zip(played_nums)
            .filter(|(_, p)| **p == 0)
            .map(|f| f.0)
            .sum();
        let win_num = self.winning_number.unwrap();
        (total_unplayed, win_num, win_num * total_unplayed)
    }
}

// Simple contiguous matrix.
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    stride1: usize,
    stride2: usize,
}

impl<T> Matrix<T>
where
    T: FromStr + Clone,
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
    pub fn from_lines(lines: Vec<String>) -> BoxResult<Self> {
        // Get number of rows, and first line
        let mut data: Vec<T> = Vec::new();
        data.extend(Matrix::parse_line(&lines[0])?);
        let cols = data.len();
        let mut rows = 1;
        for line in lines[1..].iter() {
            data.extend(Matrix::parse_line(line)?);
            rows += 1;
        }
        Ok(Matrix {
            data,
            rows,
            cols,
            stride1: 1,
            stride2: cols,
        })
    }

    fn parse_line(l: &String) -> BoxResult<Vec<T>> {
        let lines = Regex::new(r"\s+")?
            .split(l.as_str())
            .map(|x| x.parse::<T>())
            .collect::<Result<Vec<T>, T::Err>>()?;
        Ok(lines)
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
    T: FromStr + std::fmt::Display + Copy,
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_row_win() {
        let mut bingo =
            BingoBoards::from_strings(&vec![String::from("0 8\n11 5"), String::from("0 1\n2 3")])
                .unwrap();
        for m in vec![2, 3, 0] {
            if bingo.check_boards(m) {
                break;
            }
        }
        let (unplayed, winning, product) = bingo.winning_stats();
        assert_eq!(unplayed, 1);
        assert_eq!(winning, 3);
        assert_eq!(product, 3);
    }

    #[test]
    fn test_col_win() {
        let mut bingo =
            BingoBoards::from_strings(&vec![String::from("0 8\n11 5"), String::from("0 1\n2 3")])
                .unwrap();
        for m in vec![1, 3] {
            if bingo.check_boards(m) {
                break;
            }
        }
        let (unplayed, winning, product) = bingo.winning_stats();
        assert_eq!(unplayed, 2);
        assert_eq!(winning, 3);
        assert_eq!(product, 6);
    }
}
