use std::collections::BTreeSet;
use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    // Get input
    let input = fs::read_to_string(FILENAME).expect("FILE ERROR");

    // Seperate into numbers and insert into BTreeSet
    let mut joltages: BTreeSet<u32> = BTreeSet::new();
    for num in input.lines() {
        joltages.insert(num.trim().parse::<u32>().expect("PARSE COULD NOT MAKE u32"));
    }
    let mut joltages = joltages.iter();

    // Iterate through tree from lowest value
    // Store the differences in an enum
    let mut ones = 0;
    let mut threes = 0;
    let mut small = 0;

    while let Some(big) = joltages.next() {
        if big - small == 1 {
            ones += 1;
        } else if big - small == 3 {
            threes += 1;
        } else if big - small != 2 {
            panic!();
        }

        small = *big;
    }
    threes += 1;
    // Count the ones and threes then output answer
    println!("Answer: {}", ones * threes);
}
