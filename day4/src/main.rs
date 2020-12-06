use std::fs;
use std::io;
use std::io::prelude::*;

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
    let file = fs::File::open("input.txt").expect("No input found");
    let mut lines = io::BufReader::new(file).lines();

    let mut passport_details: Vec<&str> = vec![];

    loop {
        match lines.next().unwrap() {
            Ok(text) => {
                if text == "" {

                } else {
                    let mut individual_details: Vec<&str> = text.split_whitespace().collect().to_owned();
                    passport_details.append(&mut individual_details);
                }
            },
            Err(e) => panic!(e)
        }
        
    }


}


// fn parse_passport(input ) -> Passport {

// }
