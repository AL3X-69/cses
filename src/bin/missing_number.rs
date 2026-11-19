use std::{io::stdin};

fn main() {
    let mut max_s = String::new();
    let mut l = String::new();
    stdin().read_line(&mut max_s).unwrap();
    stdin().read_line(&mut l).unwrap();
    l.pop();
    max_s.pop();
    let max = max_s.parse::<i64>().unwrap();
    let mut numbers = l.split(" ")
        .map(|s: &str| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    numbers.sort();
    for n in 1..max {
        if n != numbers[(n - 1) as usize] {
            println!("{}", n);
            return;
        } 
    }

    println!("{}", numbers.last().unwrap() + 1);
}
