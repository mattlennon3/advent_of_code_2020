use std::fs;
use std::io;
use std::io::prelude::*;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
struct Passport {
    /** (Birth Year) */
    byr: String, 
    /** (Issue Year) */
    iyr: String, 
    /** (Expiration Year) */
    eyr: String, 
    /** (Height) */
    hgt: String, 
    /** (Hair Color) */
    hcl: String, 
    /** (Eye Color) */
    ecl: String, 
    /** (Passport ID) */
    pid: String, 
    /** (Country ID) */
    cid: String, 

}

// impl PartialEq for Passport {
//     fn eq(&self, other: &Self) -> bool {
//     }
// }

// impl Passport {
//     fn is_valid_part_1(&self) -> bool {
//     }
// }

fn main() {
    let file_string = fs::read_to_string("input.txt").expect("error");
    
    lazy_static! {
        static ref DOUBLE_NEWLINE : Regex = Regex::new(
                r"\n\n"
            ).unwrap();
    }

    let a: String = DOUBLE_NEWLINE.split(file_string.as_str())
        .map(|line| line.replace("\n", ""))
        .map(|line| {
            println!("{}", line);
            line
        }).collect();
}


// fn parse_passport(input ) -> Passport {

// }
