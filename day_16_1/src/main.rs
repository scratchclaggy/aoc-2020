use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let mut input_file = input_file.lines().into_iter();
    let mut ranges: Vec<Range> = vec![];
    let mut error_rate = 0;

    while let Some(line) = input_file.next() {
        if line == "" {
            break;
        }

        let ticket_field: Vec<&str> = line.split(": ").collect();
        let field_ranges: Vec<&str> = ticket_field[1].split(" or ").collect();

        for range in field_ranges {
            let range_integers: Vec<u32> = range
                .split("-")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            ranges.push((range_integers[0], range_integers[1]));
        }
    }

    let mut input_file = input_file.skip(4);

    while let Some(ticket) = input_file.next() {
        let ticket_fields: Vec<u32> = ticket
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        'outer: for field in ticket_fields {
            for i in 0..ranges.len() {
                if in_range(field, ranges[i]) {
                    // println!("{}: OK", field);
                    continue 'outer;
                }
            }
            // println!("{}: ERR", field);
            error_rate += field;
        }
    }

    println!("Answer: {}", error_rate);
}

type Range = (u32, u32);

fn in_range(val: u32, range: Range) -> bool {
    val >= range.0 && val <= range.1
}
