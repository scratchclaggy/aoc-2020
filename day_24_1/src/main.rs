use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let input_file = input_file.lines();
    let mut tiling = HashMap::<(i32, i32), bool>::new();
    let mut construction_plans = Vec::<Vec<Direction>>::new();
    for line in input_file {
        let mut plan = Vec::<Direction>::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            plan.push(match c {
                'e' => Direction::E,
                's' => match chars.next().unwrap() {
                    'e' => Direction::SE,
                    'w' => Direction::SW,
                    _ => panic!(),
                },
                'w' => Direction::W,
                'n' => match chars.next().unwrap() {
                    'e' => Direction::NE,
                    'w' => Direction::NW,
                    _ => panic!(),
                },
                _ => panic!(),
            })
        }
        construction_plans.push(plan);
    }

    for plan in construction_plans {
        let mut coordinate = (0, 0);
        for ins in plan {
            match ins {
                Direction::E => coordinate.0 += 1,
                Direction::SE => coordinate.1 += 1,
                Direction::SW => {
                    coordinate.0 -= 1;
                    coordinate.1 += 1;
                }
                Direction::W => coordinate.0 -= 1,
                Direction::NW => coordinate.1 -= 1,
                Direction::NE => {
                    coordinate.0 += 1;
                    coordinate.1 -= 1
                }
            }
        }

        if let Some(previous_value) = tiling.insert(coordinate, true) {
            if previous_value {
                tiling.insert(coordinate, false);
            }
        }
    }

    println!("Ans: {}", tiling.values().filter(|&tile| *tile).count());
}

enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
