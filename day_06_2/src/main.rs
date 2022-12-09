use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let groups = fs::read_to_string(FILENAME).expect("Error: could not open file");
    let groups: Vec<&str> = groups.split("\r\n\r\n").collect();

    let mut total = 0;

    for group in groups {
        let mut group = group.lines().map(|s| s.trim());
        let mut all_members_yes: HashSet<char> = HashSet::new();

        if let Some(party_member) = group.next() {
            for c in party_member.chars() {
                all_members_yes.insert(c);
            }
        }

        while let Some(party_member) = group.next() {
            all_members_yes.retain(|&c| party_member.contains(c));
        }

        total += all_members_yes.len();
    }

    println!("The answer is: {}\n", total);

    println!("END OF Program\n");
}
