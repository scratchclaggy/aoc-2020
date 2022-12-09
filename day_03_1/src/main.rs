use std::fs;

const FILENAME: &str = "input.txt";
const D_X: usize = 3;
//const D_Y: usize = 1;
const FIELD_SIZE: usize = 31;

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Some error with file");
    let mut x: usize = 0;
    let mut count = 0;

    for line in input.lines() {
        if line.chars().nth(x) == Some('#') {
            count += 1;
        }

        x += D_X;
        x %= FIELD_SIZE;
    }

    println!("{} trees are in your path.\n", count as u32);
}
