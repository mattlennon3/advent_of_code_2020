use std::io::prelude::*;
use std::io;
use std::fs;

fn main() {
    
    let file = fs::File::open("input.txt").expect("No input found");
    let file = io::BufReader::new(file);
    
    let numbers: Vec<u32> = file.lines()
        .map(|line| line.expect("error reading line"))
        .map(|line| line.parse::<u32>().expect("could not parse to u32"))
        .collect();
    
    let target = 2020;

    for a in &numbers {
        for b in &numbers {
            if a == b { continue };
            if (a + b) == target {
                println!("Found 2020! A: {}, B: {}", a, b);
                println!("Multiplied: {}", a * b);
                return ();
            }
        }
    }
}
