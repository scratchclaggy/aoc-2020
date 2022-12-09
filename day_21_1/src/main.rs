use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

fn parse_allergens(
    line: &str,
    map: &mut HashMap<String, HashSet<String>>,
    ingredient_count: &mut HashMap<String, usize>,
) {
    let mut line = line.split('(');
    let ingredients = line.next().unwrap();
    let allergens = line.next().unwrap();
    assert!(line.next().is_none());

    let mut ingredients_set = HashSet::new();
    ingredients
        .split(' ')
        .filter(|i| !i.is_empty())
        .for_each(|i| {
            let i = i.trim().to_string();
            if let Some(count) = ingredient_count.get_mut(&i) {
                *count += 1;
            } else {
                ingredient_count.insert(i.clone(), 1);
            }
            ingredients_set.insert(i);
        });

    let allergens = allergens
        .strip_prefix("contains")
        .and_then(|a| a.strip_suffix(')'))
        .unwrap();

    for allergen in allergens.split(',').map(|a| a.trim()) {
        if let Some(ingredients) = map.get_mut(allergen) {
            ingredients.retain(|i| ingredients_set.contains(i));
        } else {
            map.insert(allergen.to_string(), ingredients_set.clone());
        }
    }
}

fn main() {
    let file = File::open(FILENAME)
        .map(BufReader::new)
        .expect("File I/O Error");
    let mut map = HashMap::new();
    let mut ingredient_count = HashMap::new();
    for line in file.lines() {
        parse_allergens(&line.unwrap(), &mut map, &mut ingredient_count);
    }

    for (_allergen, potential_ingredients) in map.iter() {
        // println!("{}: {:?}", allergen, potential_ingredients);
        for ingredient in potential_ingredients.iter() {
            ingredient_count.remove(ingredient);
        }
    }

    println!("Ans: {}", ingredient_count.values().sum::<usize>());

    let mut ingredients_confirmed = BTreeMap::new();
    while map.len() != 0 {
        let allergen = map
            .iter()
            .skip_while(|(_, i)| i.len() != 1)
            .next()
            .unwrap()
            .0
            .clone();
        let (allergen, ingredient) = map.remove_entry(&allergen).unwrap();
        let ingredient = ingredient.into_iter().next().unwrap();
        map.values_mut()
            .for_each(|i| i.retain(|i| i != &ingredient));
        ingredients_confirmed.insert(allergen, ingredient);
    }

    for ingredient in ingredients_confirmed.values() {
        print!("{},", ingredient);
    }
    println!();
}
