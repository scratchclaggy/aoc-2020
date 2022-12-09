use std::collections::HashMap;

const INPUT: [u32; 6] = [10, 16, 6, 0, 1, 17];

fn main() {
    let mut turn_counter = 1;
    let mut current_num = 0;
    let mut spoken_numbers: HashMap<u32, u32> = HashMap::new();

    for num in INPUT.iter() {
        // println!("{}: {}", turn_counter, *num);
        spoken_numbers.insert(*num, turn_counter);
        turn_counter += 1;
    }

    while turn_counter < 30000000 {
        // println!("{}: {}", turn_counter, current_num);
        let next_num = match spoken_numbers.contains_key(&current_num) {
            true => turn_counter - *spoken_numbers.get(&current_num).unwrap(),
            false => 0,
        };
        spoken_numbers.insert(current_num, turn_counter);
        current_num = next_num;
        turn_counter += 1;
    }

    println!("Answer: {}", current_num);
}
