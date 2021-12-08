use std::fs;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
fn main() -> BoxResult<()> {
    // let input = fs::read_to_string("small_input.txt").expect("unable to read file");
    let input = fs::read_to_string("day08_input.txt").expect("unable to read file");
    let outputs = input
        .lines()
        .map(|s| s.split(" | ").collect::<Vec<&str>>()[1])
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    
    let lengths = vec![2, 3, 4, 7];
    let mut total = 0;
    for o in &outputs {
        // println!("{:?}", o);
        let sum = o.iter().map(|s| lengths.contains(&s.chars().count()) as usize).sum::<usize>();
        // println!("Sum: {}", sum);
        total += sum;
    }
    println!("{}", total);
    Ok(())
}
