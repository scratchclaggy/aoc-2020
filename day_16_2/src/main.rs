use std::cmp::Ordering;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let mut input_file = input_file.lines().into_iter();
    let mut rulebook: Vec<Rule> = vec![];
    let mut nearby_tickets: Vec<Ticket> = vec![];
    let mut found_field_name: Vec<usize> = vec![];

    while let Some(line) = input_file.next() {
        if line == "" {
            break;
        }

        let mut ticket = line.split(": ");
        let field_name = ticket.next().unwrap();
        let field_range: Vec<u32> = ticket
            .next()
            .unwrap()
            .split(" or ")
            .flat_map(|s: &str| s.split("-").map(|num| (num).parse::<u32>().unwrap()))
            .collect();

        rulebook.push(Rule::new(
            field_name,
            field_range[0],
            field_range[1],
            field_range[2],
            field_range[3],
        ));
    }

    let mut input_file = input_file.skip(1);
    let my_ticket: Ticket = input_file
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut input_file = input_file.skip(2);

    while let Some(line) = input_file.next() {
        nearby_tickets.push(line.split(",").map(|s| s.parse::<u32>().unwrap()).collect());
    }

    nearby_tickets.retain(|ticket| {
        'next_value: for value in ticket.iter() {
            for rule in rulebook.iter() {
                if in_range(*value, rule.range_1) || in_range(*value, rule.range_2) {
                    continue 'next_value;
                }
            }
            return false;
        }
        true
    });

    for rule in rulebook.iter_mut() {
        'outer: for i in 0..my_ticket.len() {
            for ticket in nearby_tickets.iter() {
                if !in_range(ticket[i], rule.range_1) && !in_range(ticket[i], rule.range_2) {
                    continue 'outer;
                }
            }
            rule.found_match(i);
        }
    }

    'outer: while found_field_name.len() < my_ticket.len() {
        rulebook.sort();
        for rule in rulebook.iter_mut() {
            rule.possible_match
                .retain(|x| !found_field_name.contains(x));
            if rule.possible_match.len() == 1 {
                rule.field_num = Some(rule.possible_match[0]);
                found_field_name.push(rule.possible_match[0]);
                continue 'outer;
            }
        }
    }
    let mut answer: u64 = 1;
    for rule in rulebook.iter() {
        if rule.ticket_field.starts_with("departure") {
            answer *= my_ticket[rule.field_num.unwrap()] as u64;
        }
    }

    println!("Answer: {}", answer);
}

struct Rule<'a> {
    ticket_field: &'a str,
    range_1: Range,
    range_2: Range,
    possible_match: Vec<usize>,
    field_num: Option<usize>,
}

impl<'a> Rule<'a> {
    fn new(ticket_field: &'a str, min_1: u32, max_1: u32, min_2: u32, max_2: u32) -> Rule<'a> {
        Rule {
            ticket_field,
            range_1: (min_1, max_1),
            range_2: (min_2, max_2),
            possible_match: vec![],
            field_num: None,
        }
    }

    fn found_match(&mut self, field_index: usize) {
        self.possible_match.push(field_index);
    }

    fn found_field(&mut self, field_index: usize) {
        self.field_num = Some(field_index);
    }
}

impl<'a> PartialEq for Rule<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.possible_match.len() == other.possible_match.len()
    }
}

impl<'a> Eq for Rule<'a> {}
impl<'a> PartialOrd for Rule<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.possible_match.len().cmp(&other.possible_match.len()))
    }
}

impl<'a> Ord for Rule<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.possible_match.len().cmp(&other.possible_match.len())
    }
}

type Range = (u32, u32);

fn in_range(val: u32, range: Range) -> bool {
    val >= range.0 && val <= range.1
}

type Ticket = Vec<u32>;
