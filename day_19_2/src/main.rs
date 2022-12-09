// My thanks to /u/sporksmith
// I was really struggling on this one, and was only able to piece it together after reviewing their
// solution.

use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";
const WORDSIZE: usize = 8;

fn main() {
    let mut rule_book: HashMap<usize, Rule> = HashMap::new();
    let mut unverified_messages: Vec<Vec<bool>> = vec![];

    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let mut input_file = input_file.split("\r\n\r\n");

    // Get ruleset
    let mut input = input_file.next().unwrap().lines();
    while let Some(line) = input.next() {
        // Rule key
        let mut rule = line.split(": ");
        let key: usize = rule.next().unwrap().parse().unwrap();

        // Rule value
        let value = rule.next().unwrap();
        if value.starts_with("\"") {
            // Literal
            if value == "\"a\"" {
                rule_book.insert(key, Rule::Literal(true));
            } else {
                rule_book.insert(key, Rule::Literal(false));
            }
        } else {
            // List
            let mut list_of_lists_vec: Vec<Vec<usize>> = vec![];
            let mut list_of_lists = value.split(" | ");
            while let Some(list) = list_of_lists.next() {
                let mut list_vec: Vec<usize> = vec![];
                for rule in list.split(" ") {
                    list_vec.push(rule.parse().unwrap());
                }
                list_of_lists_vec.push(list_vec);
            }
            rule_book.insert(key, Rule::List(list_of_lists_vec));
        }
    }

    // Get unverified messages
    let mut input = input_file.next().unwrap().lines();
    while let Some(line) = input.next() {
        let mut msg: Vec<bool> = vec![];
        for c in line.chars() {
            if c == 'a' {
                msg.push(true);
            } else {
                msg.push(false);
            }
        }
        unverified_messages.push(msg);
    }

    println!(
        "Answer: {}",
        unverified_messages
            .iter()
            .filter(|msg| check_message(&msg, &[0], &rule_book))
            .count()
    );
}

enum Rule {
    List(Vec<Vec<usize>>),
    Literal(bool),
}

fn check_message(msg: &[bool], list: &[usize], ruleset: &HashMap<usize, Rule>) -> bool {
    if list.is_empty() {
        return msg.is_empty();
    }

    match ruleset.get(&list[0]).unwrap() {
        Rule::List(sublist) => sublist
            .iter()
            .any(|sublist| check_message(&msg, &prepend(sublist, &list[1..]), &ruleset)),
        Rule::Literal(c) => {
            !msg.is_empty() && (msg[0] == *c) && check_message(&msg[1..], &list[1..], &ruleset)
        }
    }
}

fn prepend(prefix: &[usize], suffix: &[usize]) -> Vec<usize> {
    let mut v = prefix.to_owned();
    v.extend_from_slice(suffix);
    v
}
