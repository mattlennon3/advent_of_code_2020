use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

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
    cid: Option<String>, 
}

// impl PartialEq for Passport {
//     fn eq(&self, other: &Self) -> bool {
//     }
// }

// impl Passport {
//     fn is_valid_part_1(&self) -> bool {
//     }
// }

struct PassportBuilder {
    fields: HashMap<String, String>
}

impl PassportBuilder {
    fn new(input: String) -> PassportBuilder {
        let data = input.split(' ');
        for entry in data {
            let a: Vec<&str> = entry.split(":").collect();
            let (key, value) = (a[0], a[1]);
    
            // TODO: Builder pattern
    
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

        
    }

    fn add_field(&mut self, key: String, value: String) -> &mut PassportBuilder {
        self.fields.insert(key, value);
        self
    }

    fn make(&self) -> Passport {
        let passport: Passport = Passport {
            byr: self.fields.get("byr").unwrap().to_string(),
            iyr: self.fields.get("iyr").unwrap().to_string(),
            eyr: self.fields.get("eyr").unwrap().to_string(),
            hgt: self.fields.get("hgt").unwrap().to_string(),
            hcl: self.fields.get("hcl").unwrap().to_string(),
            ecl: self.fields.get("ecl").unwrap().to_string(),
            pid: self.fields.get("pid").unwrap().to_string(),
            cid: Some(self.fields.get("cid").unwrap().to_string()),
        };

        passport
    }
}


fn main() {
    let file_string = fs::read_to_string("input.txt").expect("error");
    

    let valid_quantity: u32 = file_string.split("\n\n")
        .map(|data| data.replace("\n", ""))
        .map(|data| parse_passport(data))
        .collect::<Vec<Passport>>()
        .len() as u32;
}


fn parse_passport(input: String ) -> Passport {
    // let data = input.split(' ');
    let builder = PassportBuilder::new(input);
    
    builder.make()
}
