use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    // get instructions
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");

    // seperate into lines
    // split each line around the space
    // store each instruction and variable in a vec of tuples
    let prog: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut line = line.split(" ");
            let ins = line.next().unwrap();
            let var_string = line.next().unwrap().split_at(1);
            let mut var: i32 = var_string.1.parse().unwrap();
            if var_string.0 == "-" {
                var *= -1;
            }

            (ins, var)
        })
        .collect();

    let mut executed_instructions: HashSet<usize> = HashSet::new();
    let mut instruction_pointer: usize = 0;
    let mut accumulator: i32 = 0;

    // execute while ip counter is unique
    while executed_instructions.insert(instruction_pointer) == true {
        let (ins, var) = prog[instruction_pointer];
        println!(
            "{} | acc: {}, ip: {}",
            ins, accumulator, instruction_pointer
        );
        match ins {
            "acc" => acc(&mut instruction_pointer, &mut accumulator, var),
            "jmp" => jmp(&mut instruction_pointer, var),
            "nop" => nop(&mut instruction_pointer),
            _ => panic!(),
        }
    }
    // print acc
    println!("Answer: {}\n", accumulator);
    println!("END OF PROGRAM");
}

fn acc(ip: &mut usize, acc: &mut i32, var: i32) {
    *acc += var;
    *ip += 1;
}

fn jmp(ip: &mut usize, var: i32) {
    if var.is_negative() {
        *ip -= var.wrapping_abs() as u32 as usize;
    } else {
        *ip += var as usize;
    }
}

fn nop(ip: &mut usize) {
    *ip += 1;
}
