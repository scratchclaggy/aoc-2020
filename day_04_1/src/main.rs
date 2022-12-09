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
            let kv_pair: Vec<&str> = entry.split(':').collect();
            field.insert(kv_pair[0], kv_pair[1]);
        }
        Passport { field }
    }

    fn is_okay(&self) -> bool {
        if self.field.contains_key("byr")
            && self.field.contains_key("iyr")
            && self.field.contains_key("eyr")
            && self.field.contains_key("hgt")
            && self.field.contains_key("hcl")
            && self.field.contains_key("ecl")
            && self.field.contains_key("pid")
        {
            return true;
        }
        false
    }
}
