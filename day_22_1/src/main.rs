use std::cmp;
use std::collections::VecDeque;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let mut input_file = input_file.lines().skip(1);

    let mut player_one = VecDeque::<usize>::new();
    while let Some(num) = input_file.next() {
        if num.is_empty() {
            break;
        }

        player_one.push_back(num.parse().unwrap());
    }

    let mut input_file = input_file.skip(1);
    let mut player_two = VecDeque::<usize>::new();
    while let Some(num) = input_file.next() {
        if num.is_empty() {
            break;
        }

        player_two.push_back(num.parse().unwrap());
    }

    while !player_one.is_empty() && !player_two.is_empty() {
        let one_card = player_one.pop_front().unwrap();
        let two_card = player_two.pop_front().unwrap();

        if one_card > two_card {
            player_one.push_back(one_card);
            player_one.push_back(two_card);
        } else {
            player_two.push_back(two_card);
            player_two.push_back(one_card);
        }
    }

    let mut winner = cmp::max(player_one, player_two);
    let mut i = 1;
    let mut ans = 0;
    while !winner.is_empty() {
        ans += winner.pop_back().unwrap() * i;
        i += 1;
    }

    println!("Answer: {}", ans);
}
