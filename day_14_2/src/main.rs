use num::pow;
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
                for floating_address in current_mask.decode_address(address) {
                    memory.insert(floating_address, value);
                }
            }
            Task::BitMask(next_mask) => current_mask = next_mask,
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

struct Mask {
    bit_mask: u64,
    floating_mask: u64,
    floating_indexes: Vec<usize>,
    floating_max: u64,
}

impl Mask {
    fn new(input_string: &str) -> Mask {
        let bit_mask = input_string
            .chars()
            .fold(0, |acc, b| if b == '1' { acc << 1 | 1 } else { acc << 1 });
        let floating_mask =
            input_string.chars().fold(
                u64::MAX,
                |acc, b| if b == 'X' { acc << 1 } else { acc << 1 | 1 },
            );
        let floating_indexes: Vec<usize> = input_string
            .chars()
            .enumerate()
            .filter_map(|(i, b)| if b == 'X' { Some(i) } else { None })
            .collect();
        let floating_max = pow(2, floating_indexes.len());

        Mask {
            bit_mask,
            floating_mask,
            floating_indexes,
            floating_max,
        }
    }

    fn decode_address(&self, input_address: u64) -> Vec<u64> {
        let mut address_set: Vec<u64> = vec![];
        let masked_input = input_address | self.bit_mask & self.floating_mask;
        let input_vec: &mut Vec<u8> = &mut bin_to_vec(masked_input);
        for permutation in 0..self.floating_max {
            for bit_position in 0..self.floating_indexes.len() {
                input_vec[self.floating_indexes[bit_position]] =
                    (permutation >> bit_position & 1) as u8;
            }
            address_set.push(vec_to_bin(input_vec.to_vec()));
        }
        address_set
    }
}

fn bin_to_vec(mut bin: u64) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    while bin > 0 {
        vec.push(bin as u8 & 1);
        bin = bin >> 1;
    }
    while vec.len() < 36 {
        vec.push(0);
    }
    vec.reverse();

    vec
}

fn vec_to_bin(vec: Vec<u8>) -> u64 {
    vec.into_iter().fold(0, |acc, b| acc << 1 | b as u64)
}

enum Task {
    BitMask(Mask),
    Allocation(u64, u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_test() {
        let mask = Mask::new("000000000000000000000000000000X1001X");
        let address = 42;
        let address_set = mask.decode_address(address);
        assert_eq!(
            bin_to_vec(0b10101010),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 1, 0, 1, 0, 1, 0
            ]
        );
        assert_eq!(vec_to_bin([1, 0, 1, 0, 1, 0, 1, 0].to_vec()), 0b10101010);
        assert_eq!(mask.bit_mask, 0b10010);
        assert_eq!(mask.floating_mask, !0b100001);
        assert_eq!(mask.floating_indexes, [30, 35]);
        assert_eq!(mask.floating_max, 4);
        assert_eq!(address_set, [26, 58, 27, 59]);
    }
}
