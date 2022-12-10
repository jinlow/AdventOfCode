use std::cmp;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let mut buff: VecDeque<i64> = VecDeque::new();
    buff.push_front(0);
    let mut x = 1;
    let mut signals = Vec::new();
    let mut next_check = 20;
    let mut file_iter = file.lines();
    let mut cycle = 0;

    let mut screen: Vec<Vec<&str>> = Vec::new();
    for _ in 0..6 {
        screen.push(Vec::new());
    }

    while !buff.is_empty() {
        cycle += 1;
        if (cycle == next_check) && (next_check <= 220) {
            signals.push(x * cycle);
            next_check += 40;
        }
        let draw_pixel = (cycle - 1) % 40;
        
        let sprite_dim = (cmp::max(0, x - 1), cmp::min(40, x + 1));
        let pixel = if (draw_pixel >= sprite_dim.0) && (draw_pixel <= sprite_dim.1) {
            "#"
        } else {
            "."
        };
        let row = ((cycle - 1) / 40) as usize;
        if row < 6 {
            screen[row].push(pixel);
        }
        
        if let Some(c) = buff.pop_back() {
            x += c;
        }
        
        if let Some(l) = file_iter.next() {
            let v = l.split(" ").collect::<Vec<&str>>();

            match v.get(1) {
                Some(c) => {
                    buff.push_front(c.parse::<i64>().unwrap());
                    buff.push_front(0);
                }
                None => buff.push_front(0),
            }
        }
    }
    // P1
    println!("{}", x);
    println!("{:?}", signals);
    let s_sum = signals.iter().sum::<i64>();
    println!("{}", s_sum);

    // P2
    for r in screen {
        println!("{}", r.join(""));
    }
    Ok(())
}
