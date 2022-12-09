use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let mut seat_id_vector: Vec<u32> = fs::read_to_string(FILENAME)
        .expect("Error: could not open file")
        .lines()
        .map(|s| BoardingPass::new(s).seat_id)
        .collect();

    let mut answer = 0;

    seat_id_vector.sort();
    for i in 0..(seat_id_vector.len() - 1) {
        if seat_id_vector[i] + 2 == seat_id_vector[i + 1] {
            answer = seat_id_vector[i] + 1;
        }
    }

    println!("Answer: {}", answer);
}

struct BoardingPass {
    row: u32,
    column: u32,
    seat_id: u32,
}

impl BoardingPass {
    fn new(binary_string: &str) -> BoardingPass {
        let (row, column) = binary_string.split_at(7);
        let row = bin_string_to_int(row, "F", "B");
        let column = bin_string_to_int(column, "L", "R");
        let seat_id = row * 8 + column;

        BoardingPass {
            row,
            column,
            seat_id,
        }
    }
}

fn bin_string_to_int(binary_string: &str, zero: &str, one: &str) -> u32 {
    let parsed_string = binary_string.replace(zero, "0").replace(one, "1");
    return i64::from_str_radix(&parsed_string, 2).unwrap() as u32;
}
