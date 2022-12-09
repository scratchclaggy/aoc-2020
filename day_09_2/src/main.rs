use std::fs;

const FILENAME: &str = "input.txt";
const SEARCH_VAL: i64 = 41682220;
const SEARCH_VAL_INDEX: usize = 539;

fn main() {
    // get code
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");
    let mut input: Vec<i64> = input
        .lines()
        .map(|line| line.trim().parse().expect("Couldn't convert to i64"))
        .collect();
    input.drain(SEARCH_VAL_INDEX..);

    let mut lower: usize = 0;
    let mut upper: usize = 1;
    let mut slice_width = 1;

    'outer: while upper < input.len() {
        while upper < input.len() {
            let total = input[lower..upper + 1].iter().fold(0, |acc, x| acc + x);
            if total == SEARCH_VAL {
                break 'outer;
            }
            lower += 1;
            upper += 1;
        }
        lower = 0;
        upper = slice_width;
        slice_width += 1;
    }

    let min = input[lower..upper + 1]
        .iter()
        .fold(f64::INFINITY as i64, |min, x| min.min(*x));
    let max = input[lower..upper + 1].iter().fold(0, |max, x| max.max(*x));

    println!("Answer: {}", min + max);
}
