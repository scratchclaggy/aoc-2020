use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let mut ruleset: HashMap<usize, RuleMessage> = HashMap::new();
    let mut valid_messages: Vec<Vec<bool>> = vec![vec![]];
    let mut unverified_messages: Vec<Vec<bool>> = vec![vec![]];
    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let mut input_file = input_file.lines();

    // Get ruleset
    while let Some(line) = input_file.next() {
        if line == "" {
            break;
        }

        let mut line = line.split(": ");
        let rule_index: usize = line.next().unwrap().parse().unwrap();
        let rule_contents = line.next().unwrap();
        if rule_contents.starts_with("\"") {
            // Actual value
            if rule_contents == "\"a\"" {
                ruleset.insert(rule_index, RuleMessage::Val(true));
            } else {
                ruleset.insert(rule_index, RuleMessage::Val(false));
            }
        } else {
            // Sub-rules
            let mut rule_halves: Vec<Vec<usize>> = vec![vec![]];
            let mut rule_contents = rule_contents.split(" | ");
            while let Some(rule_half) = rule_contents.next() {
                let mut half: Vec<usize> = vec![];
                for rule_reference in rule_half.split(" ") {
                    half.push(rule_reference.parse().unwrap());
                }
                rule_halves.push(half);
            }
            ruleset.insert(rule_index, RuleMessage::Rule(rule_halves));
        }
    }

    // Get unverified messages
    while let Some(line) = input_file.next() {
        let mut current_message: Vec<bool> = vec![];
        for c in line.chars() {
            if c == 'a' {
                current_message.push(true);
            } else {
                current_message.push(false);
            }
        }
        unverified_messages.push(current_message);
    }

    expand_ruleset(0, &ruleset, &mut valid_messages);
    let mut message_lengths: HashSet<usize> = HashSet::new();
    for message in unverified_messages.iter() {
        message_lengths.insert(message.len());
    }

    // debug_print(valid_messages.iter());
    // debug_print(unverified_messages.iter());

    println!(
        "Answer: {}",
        unverified_messages
            .iter()
            .filter(|msg| valid_messages.contains(*msg))
            .count()
    );
}

enum RuleMessage {
    Rule(Vec<Vec<usize>>),
    Val(bool),
}

fn expand_ruleset(
    rule: usize,
    ruleset: &HashMap<usize, RuleMessage>,
    mut messages: &mut Vec<Vec<bool>>,
) {
    match ruleset.get(&rule).unwrap() {
        RuleMessage::Rule(current_rule) => {
            // Recursive component
            // Expand the messages accoring to the current rule
            // If there is an alternative rule, give a copy and append after expansion
            let mut current_rule = current_rule.iter();
            current_rule.next();
            let mut messages_copy = messages.clone();

            if let Some(sub_rule) = current_rule.next() {
                for recursive_rule in sub_rule {
                    expand_ruleset(*recursive_rule, &ruleset, &mut messages);
                }
                // println!("First expansion: ");
                // debug_print(messages);
            }
            if let Some(sub_rule) = current_rule.next() {
                for recursive_rule in sub_rule {
                    expand_ruleset(*recursive_rule, &ruleset, &mut messages_copy);
                }
                messages.append(&mut messages_copy);
                // println!("Second expansion: ");
                // debug_print(messages);
            }
        }
        RuleMessage::Val(actual_value) => {
            // Base case
            for message in messages {
                message.push(*actual_value);
            }
        }
    }
}

fn debug_print<'a, I>(messages: I)
where
    I: Iterator<Item = &'a Vec<bool>>,
{
    for (i, message) in messages.into_iter().enumerate() {
        let message: String = message.iter().map(|c| if *c { 'a' } else { 'b' }).collect();
        println!("{}: {}", i, message);
    }
}
