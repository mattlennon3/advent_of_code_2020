use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let file = fs::File::open("input.txt").expect("No input found");
    let file = io::BufReader::new(file);

    let landscape: Vec<Vec<bool>> = file
        .lines()
        .map(|line| line.expect("error reading line"))
        .map(|line| 
            line.parse::<String>().expect("could not parse")
                .chars().map(|square| square == '#').collect()
        )
        .collect();

    let mut tree_count = 0;

    let mut x = 0;
    let mut y = 0;

    for row in landscape {
        if y == 0 { y+= 1; continue; }
        x += 3;

        let mut local_x = x;
        
        let mut find_location = true;
        
        while find_location {
            match row.iter().nth(local_x) {
                Some(is_tree) => {
                    if *is_tree { tree_count += 1 }
                    find_location = false;
                }
                None => {
                    // If we overflow to the right, we can reset our X by subtracting the length of the row
                    local_x -= row.len()
                }
            };
        }
        y += 1;
    }

    println!("{}", tree_count);
}
