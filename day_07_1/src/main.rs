use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");

    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let mut rule = line.split(" bags contain ");
        let outer = rule.next().unwrap().trim_end_matches(" bags ");
        rule = rule.next().unwrap().split(", ");
        while let Some(containers) = rule.next() {
            if containers != "no other bags." {
                let mut statement = containers.split(" bag");
                let inner = statement
                    .next()
                    .unwrap()
                    .trim_start_matches(char::is_numeric)
                    .trim();

                if !rules.contains_key(inner) {
                    rules.insert(inner, HashSet::new());
                }
                rules.get_mut(inner).unwrap().insert(outer);
            }
        }
    }

    let possibilities: &mut HashSet<&str> = &mut HashSet::new();

    count_bags("shiny gold", possibilities, &rules);

    println!("Answer: {}", possibilities.len());
}

fn count_bags<'a>(
    search: &'a str,
    possibilities: &mut HashSet<&'a str>,
    rules: &HashMap<&'a str, HashSet<&'a str>>,
) {
    if let Some(containers) = rules.get(search) {
        if !containers.is_empty() {
            for bag in containers.iter() {
                count_bags(bag, possibilities, rules);
            }
        }
    }

    possibilities.insert(search);
}
