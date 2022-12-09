use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    // P1
    // I think, we could go row-wise, and test the number of visible trees,
    // from the left, and then the right, and then do the same from the top,
    // and the bottom, there is probably a faster way...
    // As we go row-wise, we will add everything to a hashmap, to collect
    // for going column-wise...
    let mut visible_trees: HashMap<(usize, usize), i64> = HashMap::new();
    // let mut trees = HashMap::new();
    let max_rows = file.lines().count() - 1;
    let max_cols = file.lines().next().unwrap().chars().count() - 1;
    //let row_maxs = vec![0; max_rows + 1];
    let mut col_maxs = vec![-1; max_cols + 1];
    // One pass to get all of the trees visible from the top, and the bottom...
    for (i, l) in file.lines().enumerate() {
        let mut row_max = -1;
        for (j, t) in l.chars().enumerate() {
            let risk = t.to_digit(10).unwrap() as i64;
            if risk > row_max {
                visible_trees.insert((i, j), risk);
            }
            if risk > col_maxs[j] {
                visible_trees.insert((i, j), risk);
            }
            row_max = if risk > row_max { risk } else { row_max };
            col_maxs[j] = if risk > col_maxs[j] {
                risk
            } else {
                col_maxs[j]
            };
        }
    }

    let mut col_maxs = vec![-1; max_cols + 1];
    // Now check from right to left, and top to bottom.
    for (i, l) in file.lines().rev().enumerate() {
        let mut row_max = -1;
        for (j, t) in l.chars().rev().enumerate() {
            let i = max_rows - i;
            let j = max_cols - j;
            let risk = t.to_digit(10).unwrap() as i64;
            if risk > row_max {
                visible_trees.insert((i, j), risk);
            } else if risk > col_maxs[j] {
                visible_trees.insert((i, j), risk);
            }
            row_max = if risk > row_max { risk } else { row_max };
            col_maxs[j] = if risk > col_maxs[j] {
                risk
            } else {
                col_maxs[j]
            };
        }
    }

    // for i in 0..(max_rows + 1) {
    //     let mut r = String::new();
    //     for j in 0..(max_cols + 1) {
    //         let v = match visible_trees.get(&(i, j)) {
    //             Some(i) => i.to_string(),
    //             None => String::from("+"),
    //         };
    //         r += &v;
    //     }
    //     println!("{}", r);
    // }

    println!("Visible Trees {}", visible_trees.len());

    // P2
    // For P2, it seems we might need to brute force, or at least thats the most straightforward...
    // For a tree, see how far left you can view, and then how far right you can view, and then how
    // far up and down...
    let mut trees = HashMap::new();
    for (i, l) in file.lines().enumerate() {
        for (j, v) in l.chars().enumerate() {
            let risk = v.to_digit(10).unwrap() as i64;
            trees.insert((i, j), risk);
        }
    }
    let mut scores = HashMap::new();
    for ((i, j), risk) in trees.iter() {
        // Skip edge trees, as there will always be a better one...
        if (i == &0) || (j == &0) {
            continue;
        }
        // Go left...
        let mut left_score = 0;
        let mut l = *i;
        while l > 0 {
            l -= 1;
            // There is going to be at last one tree visible.
            left_score += 1;
            match trees.get(&(l, *j)) {
                Some(v) => {
                    if v < risk {
                        continue;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        // Go right
        let mut right_score = 0;
        let mut r = *i;
        while r < max_cols {
            r += 1;
            right_score += 1;
            match trees.get(&(r, *j)) {
                Some(v) => {
                    if v < risk {
                        continue;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        // Go up
        let mut up_score = 0;
        let mut u = *j;
        while u > 0 {
            u -= 1;
            up_score += 1;
            match trees.get(&(*i, u)) {
                Some(v) => {
                    if v < risk {
                        // We can keep searching, as this tree doesn't block our view...
                        continue;
                    } else {
                        // This tree blocks our view!!!
                        break;
                    }
                }
                None => break,
            }
        }

        // Go down
        let mut down_score = 0;
        let mut d = *j;
        while d < max_cols {
            d += 1;
            down_score += 1;
            match trees.get(&(*i, d)) {
                Some(v) => {
                    if v < risk {
                        continue;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        scores.insert((i, j), left_score * right_score * up_score * down_score);
    }

    println!("{:?}", scores.values().max().unwrap());
    Ok(())
}
