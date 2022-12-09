use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let answer: u32 = fs::read_to_string(FILENAME)
        .expect("Error: could not open file")
        .lines()
        .map(|s| BoardingPass::new(s).seat_id)
        .fold(0, |max, i| u32::max(max, i));

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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test() {
        let bp: Vec<BoardingPass> = INPUT.lines().map(|s| BoardingPass::new(s)).collect();
        assert_eq!(bp[0].seat_id, 567);
        assert_eq!(bp[1].seat_id, 119);
        assert_eq!(bp[2].seat_id, 820);
    }
}
