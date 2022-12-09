use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let input_file = input_file.lines();
    let mut tiling = HashSet::<(i32, i32)>::new();
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

        if !tiling.insert(coordinate) {
            tiling.remove(&coordinate);
        }
    }

    for _ in 0..100 {
        tiling = flip_tiles(&tiling);
    }

    println!("Ans: {}", tiling.len());
}

enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn adjacent_coordinates(origin: (i32, i32)) -> Vec<(i32, i32)> {
    let mut adj = Vec::<(i32, i32)>::new();
    let (x, y) = origin;
    adj.push((x + 1, y));
    adj.push((x, y + 1));
    adj.push((x - 1, y + 1));
    adj.push((x - 1, y));
    adj.push((x, y - 1));
    adj.push((x + 1, y - 1));

    adj
}

fn adjacent_black(origin: (i32, i32), tiling: &HashSet<(i32, i32)>) -> usize {
    let mut count = 0;
    for adj in adjacent_coordinates(origin) {
        if tiling.contains(&adj) {
            count += 1;
        }
    }

    count
}

fn flip_tiles(old_tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_tiles = HashSet::<(i32, i32)>::new();

    for coordinate in old_tiles.iter() {
        if adjacent_black(*coordinate, &old_tiles) == 1 {
            new_tiles.insert(*coordinate);
        }
        for adj in adjacent_coordinates(*coordinate) {
            if adjacent_black(adj, &old_tiles) == 2 {
                new_tiles.insert(adj);
            }
        }
    }

    new_tiles
}
