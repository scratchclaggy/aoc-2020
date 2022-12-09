use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");

    let mut rules: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    for line in input.lines() {
        let mut rule = line.split(" bags contain ");
        let outer = rule.next().unwrap().trim_end_matches(" bags ");
        rules.insert(outer, HashMap::new());
        rule = rule.next().unwrap().split(", ");
        while let Some(containers) = rule.next() {
            if containers != "no other bags." {
                let (quantity, inner) = containers.split(" bag").next().unwrap().split_at(2);
                let quantity = quantity.trim().parse().unwrap();

                rules.get_mut(outer).unwrap().insert(inner, quantity);
            }
        }
    }

    // for (outer, contained) in rules.iter() {
    //     println!("\'{}\':", outer);
    //     for (inner, quantity) in contained.iter() {
    //         println!("  {}: \'{}\'", quantity, inner);
    //     }
    // }

    println!("Answer: {}", count_bags("shiny gold", &rules));
}

fn count_bags(search: &str, rules: &HashMap<&str, HashMap<&str, u32>>) -> u32 {
    if let Some(containers) = rules.get(search) {
        if !containers.is_empty() {
            let mut total = 0;
            for (bag, quantity) in containers.iter() {
                total += *quantity * count_bags(*bag, &rules);
            }

            return total + 1;
        }
    }

    println!("{}", search);

    1
}
