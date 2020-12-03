use std::fs;
use std::io;
use std::io::prelude::*;

fn get_trees_in_slope(landscape: &Vec<Vec<bool>>, right: u32, down: u32) -> u32 {
    let mut tree_count = 0;

    let mut x = 0;
    let mut y = 0;

    let mut more_rows = true;

    while more_rows {
        if y == 0 { y+= down; continue; }
        x += right;
        
        // Redefine the iterator (calling .nth multiple times yields different values)
        let mut landscape_iter = landscape.iter(); // should have used https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.cycle

        match landscape_iter.nth(y as usize) {
            Some(row) => {
                let mut local_x = x;
                let mut find_location_in_row = true;
                
                while find_location_in_row {
                    match row.iter().nth(local_x as usize) {
                        Some(is_tree) => {
                            if *is_tree { tree_count += 1 }
                            find_location_in_row = false;
                        }
                        None => {
                            // If we overflow to the right, we can reset our X by subtracting the length of the row
                            local_x -= row.len() as u32
                        }
                    };
                }
            }
            None => { more_rows = false }
        };

        y += down;
    }

    tree_count
}

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

    let slopes = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut results = Vec::<u32>::new();
    let mut count = 0;
    let mut part_2_answer = 0;
    
    for slope in slopes {
        let result = get_trees_in_slope(&landscape, slope[0], slope[1]);
        println!("Slope R:{}, D: {}. Total: {}", slope[0], slope[1], result);
        results.push(result);
        
        // No vec reduce method :(
        if count > 0 {
            part_2_answer *= result;
        } else {
            part_2_answer = result;
        }
        count += 1;
    }

    println!("Multiplied: {}", part_2_answer);
}
