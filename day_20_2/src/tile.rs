use std::collections::{BTreeMap, HashMap};
use std::default::Default;

use crate::transformation::*;

pub const TILE_WIDTH: usize = 10;
pub const TILE_HEIGHT: usize = 10;

pub type TilePrimitive = [bool; TILE_WIDTH * TILE_HEIGHT];
pub type TileInsides = Box<[bool; (TILE_HEIGHT - 2) * (TILE_WIDTH - 2)]>;

pub struct Tile {
    pub sides: [u16; 8],
    pub insides: TileInsides,
    pub matching_tiles: [Option<usize>; 8],
}

impl Tile {
    pub fn new(input: &TilePrimitive) -> Tile {
        // Sides
        let mut north = 0u16;
        for i in 0..TILE_WIDTH {
            north <<= 1;
            if input[i] {
                north |= 1;
            }
        }
        let north_inverse = north.reverse_bits() >> 6;

        let mut east = 0u16;
        let mut west = 0u16;
        for row in input.chunks_exact(10) {
            east <<= 1;
            west <<= 1;
            west |= row[0] as u16;
            east |= row[TILE_WIDTH - 1] as u16;
        }
        let east_inverse = east.reverse_bits() >> 6;
        let west_inverse = west.reverse_bits() >> 6;

        let bottom_row = &input[(TILE_HEIGHT - 1) * TILE_WIDTH..];
        let south = bottom_row
            .iter()
            .fold(0u16, |acc, &arg| (acc << 1) | arg as u16);
        let south_inverse = south.reverse_bits() >> 6;

        // Insides
        let mut insides = [false; (TILE_WIDTH - 2) * (TILE_HEIGHT - 2)];
        for (src, dst) in input
            .chunks_exact(TILE_WIDTH)
            .skip(1)
            .zip(insides.chunks_exact_mut(TILE_WIDTH - 2))
        {
            dst.copy_from_slice(&src[1..TILE_WIDTH - 1]);
        }

        Tile {
            sides: [
                north,
                north_inverse,
                east,
                east_inverse,
                south,
                south_inverse,
                west,
                west_inverse,
            ],
            insides: Box::new(insides),
            matching_tiles: Default::default(),
        }
    }
}
pub fn extract_tiles(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<(usize, Tile)> {
    let mut tiles = vec![];
    let mut lines = lines.into_iter();

    loop {
        let line = lines.next().unwrap();
        let tile_info = line.as_ref();
        let tile_num = tile_info
            .strip_prefix("Tile ")
            .and_then(|s| s.strip_suffix(":"))
            .unwrap();
        let tile_num = tile_num.parse().unwrap();

        let mut tile = [false; TILE_WIDTH * TILE_HEIGHT];
        tile.chunks_exact_mut(TILE_WIDTH).for_each(|data| {
            let row_data = lines.next().unwrap();
            data.iter_mut()
                .zip(row_data.as_ref().chars())
                .for_each(|(d, c)| {
                    *d = match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("bad character {}", c),
                    }
                });
        });
        let tile = Tile::new(&tile);

        tiles.push((tile_num, tile));
        if lines.next().is_none() {
            break;
        }
    }

    tiles
}

pub fn generate_image(
    current_tile: Tile,
    x: i32,
    y: i32,
    tiles: &mut HashMap<usize, Tile>,
    image: &mut BTreeMap<(i32, i32), Tile>,
) {
    // North side
    if let Some(mut other) = current_tile.matching_tiles[0].and_then(|other| tiles.remove(&other)) {
        let idx = other
            .sides
            .iter()
            .take_while(|&&side| side != current_tile.sides[0])
            .count();
        let facing = idx / 2;
        let inverse = (idx % 2) != 0;

        other = match facing {
            //if north, flip
            0 => flip_horizontal(&other),
            //if east, rotate once and flip vertical
            1 => flip_vertical(&rotate_cw(&other)),
            //if south, no change
            2 => other,
            //if west rotate once and flip horizontal, flip vertical
            3 => flip_vertical(&flip_horizontal(&rotate_cw(&other))),
            _ => unreachable!(),
        };
        //flip around vertical to invert the south facing side
        if inverse {
            other = flip_vertical(&other);
        }
        //remove ourselves from the south facing
        other.matching_tiles[4].take().unwrap();
        other.matching_tiles[5].take().unwrap();

        //now, get other tile to check for its matches
        generate_image(other, x - 1, y, tiles, image);
    }

    // East side
    if let Some(mut other) = current_tile.matching_tiles[2].and_then(|other| tiles.remove(&other)) {
        let idx = other
            .sides
            .iter()
            .take_while(|&&side| side != current_tile.sides[2])
            .count();
        let facing = idx / 2;
        let inverse = (idx % 2) != 0;

        other = match facing {
            //if north, rotate once and flip vertical
            0 => flip_vertical(&rotate_cw(&other)),
            //if east, flip
            1 => flip_vertical(&other),
            //if south, rotate and flip horizontal
            2 => rotate_cw(&other),
            //if west,no change
            3 => other,
            _ => unreachable!(),
        };
        //flip around horizontal to invert the west facing side
        if inverse {
            other = flip_horizontal(&other);
        }
        //remove ourselves from the west facing
        other.matching_tiles[6].take().unwrap();
        other.matching_tiles[7].take().unwrap();

        //now, get other tile to check for its matches
        generate_image(other, x, y + 1, tiles, image);
    }

    // South side
    if let Some(mut other) = current_tile.matching_tiles[4].and_then(|other| tiles.remove(&other)) {
        let idx = other
            .sides
            .iter()
            .take_while(|&&side| side != current_tile.sides[4])
            .count();
        let facing = idx / 2;
        let inverse = (idx % 2) != 0;

        other = match facing {
            //if north, no change
            0 => other,
            //if east, rotate once and flip horizontal, flip vertical
            1 => flip_vertical(&flip_horizontal(&rotate_cw(&other))),
            //if south, flip horizontal
            2 => flip_horizontal(&other),
            //if west, rotate once and flip vertical
            3 => flip_vertical(&rotate_cw(&other)),
            _ => unreachable!(),
        };
        //flip around vertical to invert the north facing side
        if inverse {
            other = flip_vertical(&other);
        }
        //remove ourselves from the north facing
        other.matching_tiles[0].take().unwrap();
        other.matching_tiles[1].take().unwrap();

        //now, get other tile to check for its matches
        generate_image(other, x + 1, y, tiles, image);
    }

    // West side
    if let Some(mut other) = current_tile.matching_tiles[6].and_then(|other| tiles.remove(&other)) {
        let idx = other
            .sides
            .iter()
            .take_while(|&&side| side != current_tile.sides[6])
            .count();
        let facing = idx / 2;
        let inverse = (idx % 2) != 0;

        other = match facing {
            //if north, rotate once
            0 => rotate_cw(&other),
            //if east, no change
            1 => other,
            //if south, rotate and flip vertical
            2 => flip_vertical(&rotate_cw(&other)),
            //if west, flip vertical
            3 => flip_vertical(&other),
            _ => unreachable!(),
        };
        //flip around horizontal to invert the west facing side
        if inverse {
            other = flip_horizontal(&other);
        }
        //remove ourselves from the east facing
        other.matching_tiles[2].take().unwrap();
        other.matching_tiles[3].take().unwrap();

        //now, get other tile to check for its matches
        generate_image(other, x, y - 1, tiles, image);
    }

    assert!(image.insert((x, y), current_tile).is_none());
}

const MONSTER: [[bool; 20]; 3] = [
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true, false,
    ],
    [
        true, false, false, false, false, true, true, false, false, false, false, true, true,
        false, false, false, false, true, true, true,
    ],
    [
        false, true, false, false, true, false, false, true, false, false, true, false, false,
        true, false, false, true, false, false, false,
    ],
];

pub fn find_monsters(mut image: Vec<Vec<bool>>) -> usize {
    let mut num_monsters = 0;
    //normal
    //for rows in image.windows(3) {
    for i in 0..(image.len() - 3) {
        let mut image = image.iter_mut().skip(i);
        let r0 = &mut image.next().unwrap();
        let r1 = &mut image.next().unwrap();
        let r2 = &mut image.next().unwrap();
        //for ((r0, r1), r2) in rows[0]
        //    .windows_mut(20)
        //    .zip(rows[1].windows_mut(20))
        //    .zip(rows[2].windows_mut(20))
        for i in 0..(r0.len() - 20) {
            let r0 = &mut r0[i..(i + 20)];
            let r1 = &mut r1[i..(i + 20)];
            let r2 = &mut r2[i..(i + 20)];
            //normal
            if (r0
                .iter()
                .zip(MONSTER[0].iter())
                .filter(|&(_, t)| *t)
                .all(|(t, _)| *t)
                && r1
                    .iter()
                    .zip(MONSTER[1].iter())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t)
                && r2
                    .iter()
                    .zip(MONSTER[2].iter())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t))
            {
                num_monsters += 1;
                r0.iter_mut().for_each(|b| *b = false);
                r1.iter_mut().for_each(|b| *b = false);
                r2.iter_mut().for_each(|b| *b = false);
            }
            /*
                //flip vertical
                num_monsters += (r0
                    .iter()
                    .zip(MONSTER[0].iter().rev())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t)
                    && r1
                        .iter()
                        .zip(MONSTER[1].iter().rev())
                        .filter(|&(_, t)| *t)
                        .all(|(t, _)| *t)
                    && r2
                        .iter()
                        .zip(MONSTER[2].iter().rev())
                        .filter(|&(_, t)| *t)
                        .all(|(t, _)| *t)) as usize;
            }
            */
        }
    }
    /*
    //flip horizontal
    for rows in image.windows(3).rev() {
        for ((r0, r1), r2) in rows[0]
            .windows(20)
            .zip(rows[1].windows(20))
            .zip(rows[2].windows(20))
        {
            //normal
            num_monsters += (r0
                .iter()
                .zip(MONSTER[0].iter())
                .filter(|&(_, t)| *t)
                .all(|(t, _)| *t)
                && r1
                    .iter()
                    .zip(MONSTER[1].iter())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t)
                && r2
                    .iter()
                    .zip(MONSTER[2].iter())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t)) as usize;
            //flip vertical
            num_monsters += (r0
                .iter()
                .zip(MONSTER[0].iter().rev())
                .filter(|&(_, t)| *t)
                .all(|(t, _)| *t)
                && r1
                    .iter()
                    .zip(MONSTER[1].iter().rev())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t)
                && r2
                    .iter()
                    .zip(MONSTER[2].iter().rev())
                    .filter(|&(_, t)| *t)
                    .all(|(t, _)| *t)) as usize;
        }
    }
    */
    num_monsters
}

/*
    For each tile
    If it hasn't found a neighbor yet
        Check N, E, S, W,
            Get the Tile that matches the side we're checking
            Rotate and flip it until that tile's apropriate edge matches the edge we're checking (i.e. Nth -> Sth)
            Call fn for that tile


            (x,y):
*/

// fn print_tile(tile: &TilePrimitive) {
//     for row in tile.chunks_exact(TILE_WIDTH) {
//         for &bit in row {
//             if bit {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
// }

// impl Tile {
//     pub fn north(&self) -> u16 {
//         self.sides[0]
//     }
//     pub fn north_inverse(&self) -> u16 {
//         self.sides[1]
//     }
//     pub fn east(&self) -> u16 {
//         self.sides[2]
//     }
//     pub fn east_inverse(&self) -> u16 {
//         self.sides[3]
//     }
//     pub fn south(&self) -> u16 {
//         self.sides[4]
//     }
//     pub fn south_inverse(&self) -> u16 {
//         self.sides[5]
//     }
//     pub fn west(&self) -> u16 {
//         self.sides[6]
//     }
//     pub fn west_inverse(&self) -> u16 {
//         self.sides[7]
//     }
// }
