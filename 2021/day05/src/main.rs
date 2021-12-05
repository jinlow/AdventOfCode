use std::fs;
use std::collections::HashSet;
fn main() -> BoxResult<()>{
    // let file = fs::read_to_string("small_input.txt").expect("Unable to read.");
    let file = fs::read_to_string("day05_input.txt").expect("Unable to read.");
    let mut points_tuples = Vec::new();
    for l in file.lines() {
        points_tuples.push(read_line(l)?)
    }
    // println!("{:?}", points_tuples);
    // Part 1
    let mut points = Vec::new();
    for p in &points_tuples {
        if (p.0.0 == p.1.0) || (p.0.1 == p.1.1) {
            points.extend(expand_points(p.0, p.1));
        }
    }
    let mut visited = HashSet::new();
    let mut overlapped = HashSet::new();
    for p in &points {
        if visited.contains(&p) {
            overlapped.insert(p);
        } else {
            visited.insert(p);
        }
    }
    // println!("{:?}", points);
    println!("Number of overlapped points: {}", overlapped.len());

    // Part 2
    let mut all_points = Vec::new();
    for p in points_tuples {
        if (p.0.0 == p.1.0) || (p.0.1 == p.1.1) {
            all_points.extend(expand_points(p.0, p.1));
        } else if (p.0.0 - p.1.0).abs() == (p.0.1 - p.1.1).abs() {
            all_points.extend(expand_points_diagonal(p.0, p.1));
        }
    }
    let mut all_visited = HashSet::new();
    let mut all_overlapped = HashSet::new();
    for p in &all_points {
        if all_visited.contains(&p) {
            all_overlapped.insert(p);
        } else {
            all_visited.insert(p);
        }
    }
    println!("Number of overlapped points with digonal: {}", all_overlapped.len());

    Ok(())
}

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn read_tuple(s: &str) -> BoxResult<(i64, i64)> {
    let res = s
        .split(",")
        .map(|i| i.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    Ok((res[0], res[1]))
}

fn read_line(s: &str) -> BoxResult<((i64, i64), (i64, i64))> {
    let res = s
        .split(" -> ")
        .map(|i| read_tuple(i))
        .collect::<Result<Vec<(i64, i64)>, _>>()?;
    Ok((res[0], res[1]))
}

fn expand_points(p1: (i64, i64), p2: (i64, i64)) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    for x in calc_range(p1.0, p2.0) {
        for y in calc_range(p1.1, p2.1) {
            points.push((x, y))
        }
    }
    points
}

fn expand_points_diagonal(p1: (i64, i64), p2: (i64, i64)) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    for (x, y) in calc_range(p1.0, p2.0).iter().zip(calc_range(p1.1, p2.1)) {
        points.push((*x, y))
    }
    points
}


fn calc_range(i: i64, j: i64) -> Vec<i64> {
    if i < j {
        (i..=j).collect()
    } else {
        ((j..=i).rev()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_tuple() {
        let t = read_tuple("10,0").unwrap();
        assert_eq!(t, (10, 0))
    }
    #[test]
    fn test_read_line() {
        let t = read_line("10,0 -> 3,0").unwrap();
        assert_eq!(t, ((10, 0), (3, 0)));
    }
    #[test]
    fn test_calc_range() {
        let r1 = calc_range(0, 5);
        println!("{:?}", r1);
        assert_eq!(r1, vec![0, 1, 2, 3, 4, 5]);
        let r2 = calc_range(5, 2);
        println!("{:?}", r2);
        assert_eq!(r2, vec![5, 4, 3, 2]);
    }
    #[test]
    fn test_expand_points() {
        let p = expand_points((0, 4), (0, 1));
        assert_eq!(p, vec![(0, 4), (0, 3), (0, 2), (0, 1)]);
        let p = expand_points_diagonal((9, 7), (7, 9));
        assert_eq!(p, vec![(9, 7), (8, 8), (7, 9)]);
    }
}
