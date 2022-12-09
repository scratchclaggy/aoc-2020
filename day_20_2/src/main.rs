use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
mod tile;
mod transformation;

use tile::TILE_WIDTH;

use crate::tile::Tile;

const FILENAME: &str = "input.txt";
const MONSTER_SIZE: usize = 15;

fn main() {
    let file = File::open(FILENAME)
        .map(BufReader::new)
        .expect("File I/O Error");
    let mut tiles_no_matches = tile::extract_tiles(file.lines().map(|s| s.unwrap()));

    let mut tile_map: HashMap<usize, Tile> = HashMap::new();

    // Find the other matching tile for each edge of each tile
    while let Some((id, mut tile)) = tiles_no_matches.pop() {
        for (other_id, other_tile) in tiles_no_matches.iter_mut() {
            // For each side
            for i in 0..8 {
                // For each side in other tile
                for j in 0..8 {
                    if tile.sides[i] == other_tile.sides[j] {
                        tile.matching_tiles[i] = Some(*other_id);
                        other_tile.matching_tiles[j] = Some(id);
                    }
                }
            }
        }
        tile_map.insert(id, tile);
    }

    let mut image: BTreeMap<(i32, i32), Tile> = BTreeMap::new();
    let start_key = *tile_map.keys().next().unwrap();
    tile::generate_image(
        tile_map.remove(&start_key).unwrap(),
        0,
        0,
        &mut tile_map,
        &mut image,
    );

    assert_eq!(tile_map.len(), 0);

    let (min_x, min_y) = *image.keys().next().unwrap();

    let mut full_image: Vec<Vec<bool>> = vec![
        vec![false; (image.len() as f64).sqrt() as usize * 8];
        (image.len() as f64).sqrt() as usize * 8
    ];
    for ((x, y), tile) in image.iter() {
        for (this_x, row) in tile.insides.chunks_exact(TILE_WIDTH - 2).enumerate() {
            for (this_y, val) in row.iter().enumerate() {
                full_image[(x - min_x) as usize * 8 + this_x][(y - min_y) as usize * 8 + this_y] =
                    *val;
            }
        }
    }

    //     let full_image: Vec<Vec<bool>> = ".#.#..#.##...#.##..#####
    // ###....#.#....#..#......
    // ##.##.###.#.#..######...
    // ###.#####...#.#####.#..#
    // ##.#....#.##.####...#.##
    // ...########.#....#####.#
    // ....#..#...##..#.#.###..
    // .####...#..#.....#......
    // #..#.##..#..###.#.##....
    // #.####..#.####.#.#.###..
    // ###.#.#...#.######.#..##
    // #.####....##..########.#
    // ##..##.#...#...#.#.#.#..
    // ...#..#..#.#.##..###.###
    // .#.#....#.##.#...###.##.
    // ###.#...#..#.##.######..
    // .#.#.###.##.##.#..#.##..
    // .####.###.#...###.#..#.#
    // ..#.#..#..#.#.#.####.###
    // #..####...#.#.#.###.###.
    // #####..#####...###....##
    // #.##..#..#...#..####...#
    // .#.###..##..##..####.##.
    // ...###...##...#...#..###"
    //         .lines()
    //         .map(|line| line.chars().map(|c| c == '#').collect())
    //         .collect();

    let mut monster_count: Vec<usize> = vec![];
    monster_count.push(tile::find_monsters(full_image.clone()));
    for i in 0..3 {
        full_image = transformation::rotate_image(&full_image);
        monster_count.push(tile::find_monsters(full_image.clone()));
    }
    full_image = transformation::flip_image_horizontal(&full_image);
    monster_count.push(tile::find_monsters(full_image.clone()));
    for i in 0..3 {
        full_image = transformation::rotate_image(&full_image);
        monster_count.push(tile::find_monsters(full_image.clone()));
    }
    println!("{:?}", monster_count);

    let ans = full_image
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&b| *b)
        .count()
        - MONSTER_SIZE * monster_count.iter().max().unwrap();

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&'static str] = &[
        "Tile 2311:",
        "..##.#..#.",
        "##..#.....",
        "#...##..#.",
        "####.#...#",
        "##.##.###.",
        "##...#.###",
        ".#.#.#..##",
        "..#....#..",
        "###...#.#.",
        "..###..###",
        "",
        "Tile 1951:",
        "#.##...##.",
        "#.####...#",
        ".....#..##",
        "#...######",
        ".##.#....#",
        ".###.#####",
        "###.##.##.",
        ".###....#.",
        "..#.#..#.#",
        "#...##.#..",
        "",
        "Tile 1171:",
        "####...##.",
        "#..##.#..#",
        "##.#..#.#.",
        ".###.####.",
        "..###.####",
        ".##....##.",
        ".#...####.",
        "#.##.####.",
        "####..#...",
        ".....##...",
        "",
        "Tile 1427:",
        "###.##.#..",
        ".#..#.##..",
        ".#.##.#..#",
        "#.#.#.##.#",
        "....#...##",
        "...##..##.",
        "...#.#####",
        ".#.####.#.",
        "..#..###.#",
        "..##.#..#.",
        "",
        "Tile 1489:",
        "##.#.#....",
        "..##...#..",
        ".##..##...",
        "..#...#...",
        "#####...#.",
        "#..#.#.#.#",
        "...#.#.#..",
        "##.#...##.",
        "..##.##.##",
        "###.##.#..",
        "",
        "Tile 2473:",
        "#....####.",
        "#..#.##...",
        "#.##..#...",
        "######.#.#",
        ".#...#.#.#",
        ".#########",
        ".###.#..#.",
        "########.#",
        "##...##.#.",
        "..###.#.#.",
        "",
        "Tile 2971:",
        "..#.#....#",
        "#...###...",
        "#.#.###...",
        "##.##..#..",
        ".#####..##",
        ".#..####.#",
        "#..#.#..#.",
        "..####.###",
        "..#.#.###.",
        "...#.#.#.#",
        "",
        "Tile 2729:",
        "...#.#.#.#",
        "####.#....",
        "..#.#.....",
        "....#..#.#",
        ".##..##.#.",
        ".#.####...",
        "####.#.#..",
        "##.####...",
        "##..#.##..",
        "#.##...##.",
        "",
        "Tile 3079:",
        "#.#.#####.",
        ".#..######",
        "..#.......",
        "######....",
        "####.#..#.",
        ".#...#.##.",
        "#.#####.##",
        "..#.###...",
        "..#.......",
        "..#.###...",
    ];

    #[test]
    fn test() {
        use super::*;

        let mut tiles_no_matches = tile::extract_tiles(INPUT);

        let mut tile_map: HashMap<usize, Tile> = HashMap::new();

        while let Some((id, mut tile)) = tiles_no_matches.pop() {
            for (other_id, other_tile) in tiles_no_matches.iter_mut() {
                // For each side
                for i in 0..8 {
                    // For each side in other tile
                    for j in 0..8 {
                        if tile.sides[i] == other_tile.sides[j] {
                            tile.matching_tiles[i] = Some(*other_id);
                            other_tile.matching_tiles[j] = Some(id);
                        }
                    }
                }
            }
            tile_map.insert(id, tile);
        }

        for (id, tile) in tile_map.iter() {
            print!("{}: ", id);
            for side in tile.matching_tiles.iter() {
                match side {
                    Some(matching_side) => println!("{}", matching_side),
                    None => println!(),
                }
            }
        }
        panic!();
    }
}
