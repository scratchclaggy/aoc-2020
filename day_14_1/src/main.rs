use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let program: Vec<Task> = input
        .lines()
        .map(|line| {
            let mut input_string = line.split(" = ");
            let task_type = input_string.next().unwrap();
            let new_task: Task = match task_type {
                "mask" => Task::BitMask(Mask::new(input_string.next().unwrap())),
                _ => Task::Allocation(
                    task_type
                        .split(|c| c == '[' || c == ']')
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    input_string.next().unwrap().trim().parse::<u64>().unwrap(),
                ),
            };
            new_task
        })
        .collect();

    let mut current_mask: Mask = Mask::new("0");
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for instruction in program {
        match instruction {
            Task::Allocation(address, value) => {
                memory.insert(address, current_mask.apply_mask(value));
                //println!("{}: {}", address, current_mask.apply_mask(value));
            }
            Task::BitMask(next_mask) => current_mask = next_mask,
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

struct Mask {
    mask_zero: u64,
    mask_one: u64,
}

impl Mask {
    fn new(bin_string: &str) -> Mask {
        let mask_zero = bin_string.replace("X", "1");
        let mask_one = bin_string.replace("X", "0");

        let mask_zero = u64::from_str_radix(&mask_zero, 2).unwrap();
        let mask_one = u64::from_str_radix(&mask_one, 2).unwrap();

        Mask {
            mask_zero,
            mask_one,
        }
    }

    fn apply_mask(&self, input_num: u64) -> u64 {
        let mut output = input_num & self.mask_zero;
        output = output | self.mask_one;
        output
    }
}

enum Task {
    BitMask(Mask),
    Allocation(u64, u64),
}
