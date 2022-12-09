use std::collections::{HashSet, VecDeque};
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

    let winner = recursive_combat(&mut player_one, &mut player_two);
    let mut winning_deck = match winner {
        Winner::P1 => player_one,
        Winner::P2 => player_two,
    };

    let mut i = 1;
    let mut ans = 0;
    while !winning_deck.is_empty() {
        ans += winning_deck.pop_back().unwrap() * i;
        i += 1;
    }

    println!("Answer: {}", ans);
}

enum Winner {
    P1,
    P2,
}

fn recursive_combat(player_one: &mut VecDeque<usize>, player_two: &mut VecDeque<usize>) -> Winner {
    let mut winner = Winner::P1;
    let mut game_states = HashSet::<(Vec<usize>, Vec<usize>)>::new();

    while !player_one.is_empty() && !player_two.is_empty() {
        let current_state = (
            player_one.iter().copied().collect(),
            player_two.iter().copied().collect(),
        );
        if !game_states.insert(current_state) {
            return Winner::P1;
        }

        let one_card = player_one.pop_front().unwrap();
        let two_card = player_two.pop_front().unwrap();

        if one_card <= player_one.len() && two_card <= player_two.len() {
            let mut p_one_subset = VecDeque::<usize>::new();
            for i in 0..one_card {
                p_one_subset.push_back(player_one[i]);
            }

            let mut p_two_subset = VecDeque::<usize>::new();
            for i in 0..two_card {
                p_two_subset.push_back(player_two[i]);
            }

            winner = recursive_combat(&mut p_one_subset, &mut p_two_subset);
        } else if one_card > two_card {
            winner = Winner::P1;
        } else {
            winner = Winner::P2;
        }

        match winner {
            Winner::P1 => {
                player_one.push_back(one_card);
                player_one.push_back(two_card);
            }
            Winner::P2 => {
                player_two.push_back(two_card);
                player_two.push_back(one_card);
            }
        }
    }

    winner
}
