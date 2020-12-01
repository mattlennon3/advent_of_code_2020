use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let file = fs::File::open("input.txt").expect("No input found");
    let file = io::BufReader::new(file);

    let numbers: Vec<u32> = file
        .lines()
        .map(|line| line.expect("error reading line"))
        .map(|line| line.parse::<u32>().expect("could not parse to u32"))
        .collect();

    let target = 2020;

    for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if (a + b + c) == target {
                    println!("Found 2020! A: {}, B: {}, C: {}", a, b, c);
                    println!("Multiplied: {}", a * b * c);
                    return ();
                }
            }
        }
    }
}
