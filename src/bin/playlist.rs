
use std::{io::stdin};

fn main() {
    let mut l = String::new();
    let mut playlist = String::new();
    stdin().read_line(&mut l).unwrap();
    stdin().read_line(&mut playlist).unwrap();
    l.pop();
    playlist.pop();
    let len = l.parse::<i64>().unwrap();
    let mut ids = playlist.split(" ")
        .map(|s: &str| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    ids.sort();
    
    let mut seen = Vec::new();
    let mut r = 0;

    for n in 0..len {
        let x = ids[n as usize];
        if seen.contains(&x) {
            if seen.len() > r {
                r = seen.len();
            }
            seen.clear();

            // Speed up process if remaining < r
            if r > (len - n) as usize {
                break;
            }
        }
        seen.push(x);
    }

    println!("{}", r);
}
