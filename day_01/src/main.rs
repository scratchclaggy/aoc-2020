use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";
const SEARCH_SUM: i32 = 2020;

fn main() {
    let input_vector: Vec<i32> = fs::read_to_string(FILENAME)
        .expect("Some error with file")
        .lines()
        .map(|line| line.trim().parse().expect("Error inputting value"))
        .collect();

    let mut available: HashSet<i32> = HashSet::new();
    let mut answer: Option<(i32, i32, i32)> = Option::None;

    for term in input_vector {
        let req_sum = SEARCH_SUM - term;

        match find_terms_of_sum(req_sum, &available) {
            Some(extra_terms) => answer = Some((term, extra_terms.0, extra_terms.1)),
            None => {
                available.insert(term);
                ()
            }
        }
    }

    match answer {
        Some(answer) => println!(
            "The terms are {}, {}, and {}, and the product is {}",
            answer.0,
            answer.1,
            answer.2,
            answer.0 * answer.1 * answer.2
        ),
        None => println!("There is no answer!"),
    };
}

fn find_terms_of_sum(sum: i32, search_field: &HashSet<i32>) -> Option<(i32, i32)> {
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
