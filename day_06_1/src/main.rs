use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let groups = fs::read_to_string(FILENAME).expect("Error: could not open file");
    let groups: Vec<&str> = groups.split("\r\n\r\n").collect();

    let mut total = 0;

    for group in groups {
        let group: Vec<char> = group.chars().filter(|c| c.is_alphabetic()).collect();
        let mut answered_yes: HashSet<char> = HashSet::new();
        for c in group {
            answered_yes.insert(c);
        }

        total += answered_yes.len();
    }

    println!("The answer is: {}\n", total);

    println!("END OF Program\n");
}
