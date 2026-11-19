use std::{io::stdin};

fn main() {
    let mut dna = String::new();
    stdin().read_line(&mut dna).unwrap();
    dna.pop();
    
    let mut max = 0;
    let mut i = 0;
    let mut last_c = '\0';

    for c in dna.chars() {
        if c != last_c {
            if i > max {
                max = i;
            }
            i = 0;
            last_c = c;
        }
        i += 1;
    }

    println!("{}", if i > max { i } else { max });
}
