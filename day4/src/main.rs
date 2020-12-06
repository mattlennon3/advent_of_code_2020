use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Passport {
    /** (Birth Year) */
    byr: Option<String>, 
    /** (Issue Year) */
    iyr: Option<String>, 
    /** (Expiration Year) */
    eyr: Option<String>, 
    /** (Height) */
    hgt: Option<String>, 
    /** (Hair Color) */
    hcl: Option<String>, 
    /** (Eye Color) */
    ecl: Option<String>, 
    /** (Passport ID) */
    pid: Option<String>, 
    /** (Country ID) */
    cid: Option<String>, 
}

impl Passport {
    fn empty() -> Passport {
        Passport {
            byr: None,
            iyr: None, 
            eyr: None, 
            hgt: None, 
            hcl: None, 
            ecl: None, 
            pid: None, 
            cid: None
        }
    }
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
    

    let valid_quantity: u32 = file_string.split("\n\n")
        .map(|line| line.replace("\n", ""))
        .map(|line| parse_passport(line))
        .collect::<Vec<Passport>>()
        .len() as u32;
}


fn parse_passport(input: String ) -> Passport {
    let data = input.split(' ');
    let passport: Passport = Passport::empty();

    for entry in data {
        let a: Vec<&str> = entry.split(":").collect();
        let (key, value) = (a[0], a[1]);


        match key {
            "byr" => passport.byr = Some(value.to_string()),
            "iyr" => passport.iyr = Some(value.to_string()),
            "eyr" => passport.eyr = Some(value.to_string()),
            "hgt" => passport.hgt = Some(value.to_string()),
            "hcl" => passport.hcl = Some(value.to_string()),
            "ecl" => passport.ecl = Some(value.to_string()),
            "pid" => passport.pid = Some(value.to_string()),
            "cid" => passport.cid = Some(value.to_string()),
            _ => panic!()            
        }

    }

    passport
}
