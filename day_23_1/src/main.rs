use std::collections::VecDeque;

fn main() {
    let input = "364297581";
    let mut cups: VecDeque<u32> = input.chars().map(|num| num.to_digit(10).unwrap()).collect();

    for _ in 0..100 {
        cups = crab_cups(&mut cups);
    }

    println!("{:?}", cups);
}

fn crab_cups(mut cups: &mut VecDeque<u32>) -> VecDeque<u32> {
    let current = cups.pop_front().unwrap();
    let mut picked_up = VecDeque::<u32>::new();
    for _ in 0..3 {
        picked_up.push_back(cups.pop_front().unwrap());
    }

    let mut destination = current;

    while !cups.contains(&destination) {
        destination = destination.saturating_sub(1);
        if destination == 0 {
            destination = 9;
        }
    }

    let mut shuffled = VecDeque::<u32>::new();
    while let Some(next) = cups.pop_front() {
        shuffled.push_back(next);
        if next == destination {
            shuffled.append(&mut picked_up);
            break;
        }
    }
    shuffled.append(&mut cups);
    shuffled.push_back(current);

    shuffled
}
