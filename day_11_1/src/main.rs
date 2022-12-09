use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let input = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let input = input.split("\r\n").collect::<Vec<&str>>();

    let width = input[0].len();
    let mut current_seating: Vec<Seat> = Vec::new();
    let mut past_seating: Vec<Seat> = Vec::new();

    for line in input {
        for c in line.chars() {
            if c == 'L' {
                current_seating.push(Seat::Vacant);
            } else if c == '.' {
                current_seating.push(Seat::Floor);
            } else {
                panic!("UNEXPECTED CHAR IN INPUT: {}", c);
            }
        }
    }

    while current_seating != past_seating {
        past_seating = current_seating.to_vec();
        current_seating = new_seating_vec(&current_seating, width);
    }

    let occupied_seats = current_seating
        .iter()
        .filter(|seat| **seat == Seat::Occupied)
        .count();

    println!("Answer: {}", occupied_seats);
}

fn new_seating_vec(current_seating: &Vec<Seat>, width: usize) -> Vec<Seat> {
    let mut new_seating: Vec<Seat> = Vec::new();
    for i in 0..current_seating.len() {
        match current_seating[i] {
            Seat::Vacant => {
                if count_adjacent(i, &current_seating, width) == 0 {
                    new_seating.push(Seat::Occupied);
                } else {
                    new_seating.push(Seat::Vacant);
                }
            }
            Seat::Occupied => {
                if count_adjacent(i, &current_seating, width) >= 4 {
                    new_seating.push(Seat::Vacant);
                } else {
                    new_seating.push(Seat::Occupied);
                }
            }
            Seat::Floor => new_seating.push(Seat::Floor),
        }
    }

    new_seating
}

fn count_adjacent(seat: usize, seat_arrangement: &Vec<Seat>, width: usize) -> u32 {
    let mut occupied_seats = 0;
    let x_origin = seat % width;
    let y_origin = seat / width;

    for x_adjacent in -1..2 {
        let x_adjacent = x_origin as i32 + x_adjacent;
        if x_adjacent >= 0 && x_adjacent < width as i32 {
            for y_adjacent in -1..2 {
                let y_adjacent = y_origin as i32 + y_adjacent;
                if y_adjacent >= 0 {
                    let adjacent_seat = y_adjacent as usize * width + x_adjacent as usize;
                    if adjacent_seat != seat && adjacent_seat < seat_arrangement.len() {
                        if seat_arrangement[adjacent_seat] == Seat::Occupied {
                            occupied_seats += 1;
                        }
                    }
                }
            }
        }
    }

    occupied_seats
}

#[derive(PartialEq, Copy, Clone)]
enum Seat {
    Vacant,
    Occupied,
    Floor,
}
