use std::{io::stdin};

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.pop();
    let mut n = s.parse::<i64>().unwrap();
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        s += " ";
        s += &n.to_string();
    }
    s += " ";
    println!("{}", s);
}
