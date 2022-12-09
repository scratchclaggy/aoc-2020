use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    // Get input, add zero, sort, reverse
    let mut joltages = fs::read_to_string(FILENAME)
        .expect("FILE ERROR")
        .lines()
        .map(|num| num.trim().parse().expect("COULD NOT PARSE"))
        .collect::<Vec<u32>>();
    joltages.push(0);
    joltages.sort();
    joltages.reverse();

    let mut adaptor_pathways: Vec<u64> = Vec::new();

    // There can only be one path to end from the last two elements
    adaptor_pathways.push(1);
    adaptor_pathways.push(1);

    // There can be either one or two pathways from third last to last
    if joltages[2] >= joltages[0] - 3 {
        adaptor_pathways.push(2);
    } else {
        adaptor_pathways.push(1);
    }

    // Iterate through all other elements, adding cumulative total of pathways
    for i in 3..joltages.len() {
        // Can always access next element
        let mut pathways_to_end = adaptor_pathways[i - 1];

        // If it can access the element after next
        if joltages[i] + 3 >= joltages[i - 2] {
            pathways_to_end += adaptor_pathways[i - 2];

            // The element after that
            if joltages[i] + 3 >= joltages[i - 3] {
                pathways_to_end += adaptor_pathways[i - 3];
            }
        }

        adaptor_pathways.push(pathways_to_end);
    }

    println!("Answer: {}", adaptor_pathways.last().unwrap());
}
