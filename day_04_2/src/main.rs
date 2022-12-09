use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");

    let answer: usize = input
        .split("\r\n\r\n")
        .filter(|s| Passport::new(s).is_okay())
        .count();

    println!("The answer is: {}\n", answer);

    println!("END OF Program\n");
}

struct Passport<'a> {
    field: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn new(passport_string: &'a str) -> Passport {
        let mut field: HashMap<&str, &str> = HashMap::new();
        for entry in passport_string.split(|c| c == ' ' || c == '\n') {
            let kv_pair: Vec<&str> = entry.split(':').map(|s| s.trim()).collect();
            field.insert(kv_pair[0], kv_pair[1]);
        }
        Passport { field }
    }

    fn is_okay(&self) -> bool {
        // println!("byr: {}", self.byr_okay());
        // println!("iyr: {}", self.iyr_okay());
        // println!("eyr: {}", self.eyr_okay());
        // println!("hgt: {}", self.hgt_okay());
        // println!("hcl: {}", self.hcl_okay());
        // println!("ecl: {}", self.ecl_okay());
        // println!("pid: {}", self.pid_okay());
        // println!();

        if self.byr_okay()
            && self.iyr_okay()
            && self.eyr_okay()
            && self.hgt_okay()
            && self.hcl_okay()
            && self.ecl_okay()
            && self.pid_okay()
        {
            return true;
        }
        false
    }

    fn byr_okay(&self) -> bool {
        if let Some(byr) = self.field.get("byr") {
            return byr
                .parse::<u32>()
                .map(|byr| byr >= 1920 && byr <= 2002)
                .unwrap_or(false);
        }

        false
    }

    fn iyr_okay(&self) -> bool {
        if let Some(iyr) = self.field.get("iyr") {
            return iyr
                .parse::<u32>()
                .map(|iyr| iyr >= 2010 && iyr <= 2020)
                .unwrap_or(false);
        }

        false
    }

    fn eyr_okay(&self) -> bool {
        if let Some(eyr) = self.field.get("eyr") {
            return eyr
                .parse::<u32>()
                .map(|eyr| eyr >= 2020 && eyr <= 2030)
                .unwrap_or(false);
        }

        false
    }

    fn hgt_okay(&self) -> bool {
        if let Some(hgt) = self.field.get("hgt") {
            if hgt.len() > 3 {
                let (height, unit) = hgt.split_at(hgt.len() - 2);
                if let Ok(height) = height.parse::<u32>() {
                    return match unit {
                        "cm" => height >= 150 && height <= 193,
                        "in" => height >= 59 && height <= 76,
                        _ => false,
                    };
                }
            }
        }
        false
    }

    fn hcl_okay(&self) -> bool {
        if let Some(hcl) = self.field.get("hcl") {
            let (c, number) = hcl.split_at(1);
            if c == "#" {
                return match i64::from_str_radix(number, 16) {
                    Ok(_) => true,
                    Err(_) => false,
                };
            }
        }

        false
    }

    fn ecl_okay(&self) -> bool {
        if let Some(ecl) = self.field.get("ecl") {
            return "amb blu brn gry grn hzl oth".contains(ecl);
        }
        false
    }

    fn pid_okay(&self) -> bool {
        if let Some(pid) = self.field.get("pid") {
            if pid.len() == 9 {
                return pid.chars().all(char::is_numeric);
            }
        }
        false
    }
}
