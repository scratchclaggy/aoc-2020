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
        println!(
            "x: {}, y: {} | x: {}, y: {}",
            boat.position.0, boat.position.1, boat.waypoint.0, boat.waypoint.1
        );
    }

    println!("Answer = {}", boat.distance_traveled())
}

struct Boat {
    position: (i32, i32),
    waypoint: (i32, i32),
}

type Instruction = (char, u32);

impl Boat {
    //                                   E       N        W       S
    const BEARINGS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn new() -> Boat {
        let position = (0, 0);
        let waypoint = (10, 1);

        Boat { position, waypoint }
    }

    fn navigate(&mut self, instruction: Instruction) {
        let (action, value) = instruction;
        match action {
            'N' => self.translate(Boat::BEARINGS[1], value),
            'E' => self.translate(Boat::BEARINGS[0], value),
            'S' => self.translate(Boat::BEARINGS[3], value),
            'W' => self.translate(Boat::BEARINGS[2], value),
            'L' => self.rotate(true, value),
            'R' => self.rotate(false, value),
            'F' => self.travel(value),
            _ => panic!("Invalid action"),
        }
    }

    fn travel(&mut self, distance: u32) {
        self.position.0 += self.waypoint.0 * distance as i32; // x
        self.position.1 += self.waypoint.1 * distance as i32; // y
    }

   fn translate(&mut self, bearing: (i32, i32), distance: u32) {
        self.waypoint.0 += bearing.0 * distance as i32; // x
        self.waypoint.1 += bearing.1 * distance as i32; // y
    }

    fn rotate(&mut self, counter_clockwise: bool, angle: u32) {
        let mut angle: f64 = (angle as f64).to_radians();
        if !counter_clockwise {
            angle = angle.copysign(-1.0);
        }
        println!(
            "{}, cos = {}, sin = {}",
            angle,
            angle.cos().round(),
            angle.sin().round()
        );
        self.waypoint.0 = self.waypoint.0 * angle.cos().round() as i32
            - self.waypoint.1 * angle.sin().round() as i32;
        self.waypoint.1 = self.waypoint.0 * angle.sin().round() as i32
            + self.waypoint.1 * angle.cos().round() as i32;
    }

    fn distance_traveled(&self) -> u32 {
        (i32::abs(self.position.0) + i32::abs(self.position.1)) as u32
    }
}
