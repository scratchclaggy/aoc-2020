use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let course: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let (c, num) = line.trim().split_at(1);
            let c = c.chars().next().unwrap();
            let num: u32 = num.parse().unwrap();
            (c, num)
        })
        .collect();
    let mut boat = Boat::new();

    for instruction in course {
        boat.navigate(instruction);
    }

    println!("Answer = {}", boat.distance_traveled())
}

struct Boat {
    position: (i32, i32),
    bearing: usize,
}

type Instruction = (char, u32);

impl Boat {
    //                                   E       N        W       S
    const BEARINGS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn new() -> Boat {
        let position = (0, 0);
        let bearing = 0; // East-bound

        Boat { position, bearing }
    }

    fn navigate(&mut self, instruction: Instruction) {
        let (action, value) = instruction;
        match action {
            'N' => self.travel(Boat::BEARINGS[1], value),
            'E' => self.travel(Boat::BEARINGS[0], value),
            'S' => self.travel(Boat::BEARINGS[3], value),
            'W' => self.travel(Boat::BEARINGS[2], value),
            'L' => self.rotate(true, value),
            'R' => self.rotate(false, value),
            'F' => self.travel(Boat::BEARINGS[self.bearing], value),
            _ => panic!("Invalid action"),
        }
    }

    fn travel(&mut self, bearing: (i32, i32), distance: u32) {
        self.position.0 += bearing.0 * distance as i32; // x
        self.position.1 += bearing.1 * distance as i32; // y
    }

    fn rotate(&mut self, counter_clockwise: bool, value: u32) {
        let quater_turns = value as usize / 90;
        if counter_clockwise {
            self.bearing = (self.bearing + quater_turns) % 4;
        } else {
            self.bearing = (self.bearing + (4 - (quater_turns % 4))) % 4;
        }
    }

    fn distance_traveled(&self) -> u32 {
        (i32::abs(self.position.0) + i32::abs(self.position.1)) as u32
    }
}
