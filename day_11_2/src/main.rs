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
        // print_seating(&current_seating, width);
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
                if count_adjacent(i, &current_seating, width) >= 5 {
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

    // Check North
    let mut adjacent_seat = seat;
    while let Some(_) = adjacent_seat.checked_sub(width) {
        adjacent_seat -= width;

        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }
    }

    // Check South
    adjacent_seat = seat + width;
    while adjacent_seat < seat_arrangement.len() {
        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }

        adjacent_seat += width;
    }

    // Check East
    adjacent_seat = seat + 1;
    while adjacent_seat % width != 0 {
        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }

        adjacent_seat += 1;
    }

    // Check West
    adjacent_seat = seat;
    while let Some(_) = adjacent_seat.checked_sub(1) {
        adjacent_seat -= 1;

        if adjacent_seat % width == width - 1 {
            break;
        }

        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }
    }

    // Check NE
    adjacent_seat = seat;
    while let Some(_) = adjacent_seat.checked_sub(width - 1) {
        adjacent_seat -= width - 1;

        if adjacent_seat % width == 0 {
            break;
        }

        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }
    }

    // Check NW
    adjacent_seat = seat;
    while let Some(_) = adjacent_seat.checked_sub(width + 1) {
        adjacent_seat -= width + 1;
        if adjacent_seat % width == width - 1 {
            break;
        }

        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }
    }

    // Check SE
    adjacent_seat = seat + width + 1;
    while adjacent_seat < seat_arrangement.len() {
        if adjacent_seat % width == 0 {
            break;
        }
        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }

        adjacent_seat += width + 1;
    }

    // Check SW
    adjacent_seat = seat + width - 1;
    while adjacent_seat < seat_arrangement.len() {
        if adjacent_seat % width == width - 1 {
            break;
        }
        match seat_arrangement[adjacent_seat] {
            Seat::Occupied => {
                occupied_seats += 1;
                break;
            }
            Seat::Vacant => break,
            Seat::Floor => {}
        }

        adjacent_seat += width - 1;
    }

    occupied_seats
}

fn print_seating(seating: &Vec<Seat>, width: usize) {
    for i in 0..seating.len() {
        let seat = match seating[i] {
            Seat::Occupied => '#',
            Seat::Vacant => 'L',
            Seat::Floor => '.',
        };
        print!("{}", seat);

        if i % width == width - 1 {
            println!();
        }
    }

    println!();
}

#[derive(PartialEq, Copy, Clone)]
enum Seat {
    Vacant,
    Occupied,
    Floor,
}
