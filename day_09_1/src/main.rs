use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

const FILENAME: &str = "input.txt";
const PREAMBLE_LEN: usize = 25;

fn main() {
    // get code
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");
    let input: Vec<i64> = input
        .lines()
        .map(|line| line.trim().parse().expect("Couldn't convert to i64"))
        .collect();
    let mut buffer: VecDeque<i64> = VecDeque::new();

    // fill buffer
    for i in 0..PREAMBLE_LEN {
        buffer.push_back(input[i]);
    }

    for i in PREAMBLE_LEN..input.len() {
        if let Some(_answer) = find_terms_of_sum(input[i], &buffer) {
            buffer.pop_front();
            buffer.push_back(input[i]);
        } else {
            println!("Answer: {} at index {}", input[i], i);
            break;
        }
    }

    println!("\nEND OF PROGRAM");
}

fn find_terms_of_sum(sum: i64, search_field: &VecDeque<i64>) -> Option<(i64, i64)> {
    let mut available = HashSet::new();

    for num in search_field {
        let req_num = sum - num;

        if available.contains(&req_num) {
            return Some((*num, req_num));
        } else {
            available.insert(num);
        }
    }

    None
}
