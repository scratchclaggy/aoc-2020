use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let mut infinicube: HashMap<Cube, bool> = HashMap::new();

    // Collect positions of active cube in start state
    let input_vec = fs::read_to_string(FILENAME).expect("File I/O error");
    let input_vec: Vec<Vec<bool>> = input_vec
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect()
        })
        .collect();

    for i in 0..input_vec.len() {
        for j in 0..input_vec[i].len() {
            // Insert start state into the infinicube
            if input_vec[i][j] {
                insert_new_active_cube((0, 0, i as i32, j as i32), &mut infinicube);
            } else {
                infinicube.insert((0, 0, i as i32, j as i32), false);
            }
        }
    }
    println!("0: {}", infinicube.values().filter(|cube| **cube).count());

    // For each point in the infinicube, check the # of active adjacent cubes
    // and store the required state in a new infinicube
    for _iteration in 0..6 {
        let mut next_infinicube: HashMap<Cube, bool> = HashMap::new();

        for cube in infinicube.keys() {
            let active_neighbors = active_adjacent_cubes(*cube, &infinicube);
            match infinicube.get(cube) {
                Some(true) => {
                    if active_neighbors == 2 || active_neighbors == 3 {
                        insert_new_active_cube(*cube, &mut next_infinicube);
                    } else {
                        next_infinicube.insert(*cube, false);
                    }
                }
                Some(false) => {
                    if active_neighbors == 3 {
                        insert_new_active_cube(*cube, &mut next_infinicube);
                    } else {
                        next_infinicube.insert(*cube, false);
                    }
                }
                None => (),
            }
        }

        infinicube = next_infinicube;
        println!(
            "{}: {}",
            _iteration + 1,
            infinicube.values().filter(|cube| **cube).count()
        );
    }
}

type Cube = (i32, i32, i32, i32);

fn adjacent_to(origin: Cube) -> Vec<Cube> {
    let mut adjacent_points: Vec<Cube> = vec![];

    for w in (origin.0) - 1..(origin.0) + 2 {
        for x in (origin.1) - 1..(origin.1) + 2 {
            for y in (origin.2) - 1..(origin.2) + 2 {
                for z in (origin.3) - 1..(origin.3) + 2 {
                    adjacent_points.push((w, x, y, z));
                }
            }
        }
    }
    adjacent_points.remove(40); // Remove origin from list of adjacent points
    adjacent_points
}

fn active_adjacent_cubes(origin: Cube, infinicube: &HashMap<Cube, bool>) -> usize {
    let mut active_count = 0;
    for cube in adjacent_to(origin).iter() {
        match infinicube.get(cube) {
            Some(true) => active_count += 1,
            _ => (),
        }
    }
    active_count
}

fn insert_new_active_cube(location: Cube, infinicube: &mut HashMap<Cube, bool>) {
    infinicube.insert(location, true);
    for cube in adjacent_to(location).iter() {
        if !infinicube.contains_key(cube) {
            infinicube.insert(*cube, false);
        }
    }
}
