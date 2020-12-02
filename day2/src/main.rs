use std::fs;
use std::io;
use std::io::prelude::*;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
struct PasswordEntry {
    lower: u32,
    upper: u32,
    letter: char,
    password: String,
}

impl PartialEq for PasswordEntry {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower &&
        self.upper == other.upper &&
        self.letter == other.letter &&
        self.password == other.password
    }
}

impl PasswordEntry {
    fn is_valid_part_1(&self) -> bool {
        let length: u32 = self.password.chars()
            .filter(|character| character == &self.letter).count() as u32;

        length >= self.lower && length <= self.upper
    }   

    fn is_valid_part_2(&self) -> bool {
        let lower_match = self.letter == self.password.chars().nth((self.lower - 1) as usize).unwrap();
        let upper_match = self.letter == self.password.chars().nth((self.upper - 1) as usize).unwrap();

        (lower_match && !upper_match) || (!lower_match && upper_match)
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("No input found");
    let file = io::BufReader::new(file);

    let passwords: Vec<PasswordEntry> = file
        .lines()
        .map(|line| line.expect("error reading line"))
        .map(|line| parse_password(line.as_str()))
        .collect();

    let part_1_result: Vec<PasswordEntry> = passwords
        .clone()
        .into_iter()
        .filter(|entry| entry.is_valid_part_1())
        .collect();

    let part_2_result: Vec<PasswordEntry> = passwords
        .clone()
        .into_iter()
        .filter(|entry| entry.is_valid_part_2())
        .collect();


    println!("Part 1: {}", part_1_result.len());

    println!("Part 2: {}", part_2_result.len());
}

fn parse_password(input: &str) -> PasswordEntry {
    lazy_static! {
        static ref PASSWORD_ENTRY_REGEX : Regex = Regex::new(
                r"(\d+)-(\d+)\s(.)[:]\s(\w+)"
            ).unwrap();
    }

    let entry = match PASSWORD_ENTRY_REGEX.captures(input) {
        Some(result) => {
            PasswordEntry {
                lower: result.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                upper: result.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                letter: result.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                password: result.get(4).unwrap().as_str().to_string(),
            }
        },
        _ => panic!()
    };

    entry
}



#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn one_instance() {
        assert_eq!(parse_password("1-3 a: abcde"), PasswordEntry {
            lower: 1,
            upper: 3,
            letter: 'a',
            password: "abcde".to_string(),
        });
    }
    
    #[test]
    fn one_instance_validity() {
        assert_eq!(PasswordEntry {
            lower: 1,
            upper: 3,
            letter: 'a',
            password: "abcde".to_string(),
        }.is_valid_part_1(), true);
    }
    
    #[test]
    fn zero_instances() {
        assert_eq!(parse_password("1-3 b: cdefg"), PasswordEntry {
            lower: 1,
            upper: 3,
            letter: 'b',
            password: "cdefg".to_string(),
        });
    }
    
    #[test]
    fn zero_instances_validity() {
        assert_eq!(PasswordEntry {
            lower: 1,
            upper: 3,
            letter: 'b',
            password: "cdefg".to_string(),
        }.is_valid_part_1(), false);
    }
    
    #[test]
    fn max_instances() {
        assert_eq!(parse_password("2-9 c: ccccccccc"), PasswordEntry {
            lower: 2,
            upper: 9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        });
    }
    
    #[test]
    fn max_instances_validity() {
        assert_eq!(PasswordEntry {
            lower: 2,
            upper: 9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        }.is_valid_part_1(), true);
    }

}



#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn one_position_is_valid() {
        assert_eq!(PasswordEntry {
            lower: 1,
            upper: 3,
            letter: 'a',
            password: "abcde".to_string(),
        }.is_valid_part_2(), true);
    }

    #[test]
    fn no_position_is_invalid() {
        assert_eq!(PasswordEntry {
            lower: 1,
            upper: 3,
            letter: 'b',
            password: "cdefg".to_string(),
        }.is_valid_part_2(), false);
    }

    #[test]
    fn multiple_positons_invalid() {
        assert_eq!(PasswordEntry {
            lower: 2,
            upper: 9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        }.is_valid_part_2(), false);
    }
}
