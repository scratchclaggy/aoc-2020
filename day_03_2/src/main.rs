use std::fs;

const FILENAME: &str = "input.txt";
const FIELD_SIZE: usize = 31;

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Some error with file");
    let mut trees: Vec<u32> = Vec::new();
    trees.push(trees_encountered(&input, FIELD_SIZE, 1, 1));
    trees.push(trees_encountered(&input, FIELD_SIZE, 3, 1));
    trees.push(trees_encountered(&input, FIELD_SIZE, 5, 1));
    trees.push(trees_encountered(&input, FIELD_SIZE, 7, 1));
    trees.push(trees_encountered(&input, FIELD_SIZE, 1, 2));

    let mut product = 1;

    print!("Your terms are: ");
    for term in trees {
        print!("{} ", term);
        product *= term;
    }
    println!();

    println!("The product is {}.\n", product);
}

fn trees_encountered(field: &String, field_width: usize, dx: usize, dy: usize) -> u32 {
    let mut x: usize = 0;
    let mut count: u32 = 0;

    for (y, line) in field.lines().enumerate() {
        if y % dy == 0 {
            if line.chars().nth(x) == Some('#') {
                count += 1;
            }

            x += dx;
            x %= field_width;
        }
    }
    count
}
