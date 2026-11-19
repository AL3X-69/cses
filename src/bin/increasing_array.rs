use std::{io::stdin};

fn main() {
    let _ = stdin().read_line(&mut String::new());
    let mut array = String::new();
    stdin().read_line(&mut array).unwrap();
    array.pop();

    let numbers = array.split(" ")
        .map(|s|s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut r = 0;
    let mut prev = numbers[0];
    for i in 1..numbers.len() {
        let n = numbers[i];
        let delta = if prev > n { prev - n } else { 0 };
        r += delta;
        prev = n + delta;
    }
    
    println!("{}", r);
}

