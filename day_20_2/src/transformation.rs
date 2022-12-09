use crate::tile::{Tile, TileInsides, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum Transform {
    None,
    FlipH,
    FlipV,
    FlipBoth,
}

/*
    0: North
    1: North inv
    2: East
    3: East inv
    4: South
    5: South inv
    6: West
    7: West inv
*/
fn flip_horizontal_insides(tile_primitive: &TileInsides) -> TileInsides {
    let mut out = Box::new([false; (TILE_HEIGHT - 2) * (TILE_WIDTH - 2)]);
    tile_primitive
        .chunks_exact(TILE_WIDTH - 2)
        .zip(out.chunks_exact_mut(TILE_WIDTH - 2).rev())
        .for_each(|(t, o)| o.copy_from_slice(t));
    out
}

pub fn flip_horizontal(old: &Tile) -> Tile {
    Tile {
        sides: [
            old.sides[4], // South -> North
            old.sides[5], // South inv -> North inv
            old.sides[3], // East inv -> East
            old.sides[2], // East -> East inv
            old.sides[0], // North -> South
            old.sides[1], // North inv -> South inv
            old.sides[7], // West inv -> West
            old.sides[6], // West -> West inv
        ],
        insides: flip_horizontal_insides(&old.insides),
        matching_tiles: [
            old.matching_tiles[4], // South -> North
            old.matching_tiles[5], // South inv -> North inv
            old.matching_tiles[3], // East inv -> East
            old.matching_tiles[2], // East -> East inv
            old.matching_tiles[0], // North -> South
            old.matching_tiles[1], // North inv -> South inv
            old.matching_tiles[7], // West inv -> West
            old.matching_tiles[6], // West -> West inv
        ],
    }
}

fn flip_vertical_insides(tile_primitive: &TileInsides) -> TileInsides {
    let mut out = Box::new([false; (TILE_HEIGHT - 2) * (TILE_WIDTH - 2)]);
    tile_primitive
        .chunks_exact(TILE_WIDTH - 2)
        .zip(out.chunks_exact_mut(TILE_WIDTH - 2))
        .for_each(|(t, o)| t.iter().zip(o.iter_mut().rev()).for_each(|(t, o)| *o = *t));
    out
}

pub fn flip_vertical(old: &Tile) -> Tile {
    Tile {
        sides: [
            old.sides[1], // North -> North inv
            old.sides[0], // North inv -> North
            old.sides[6], // West -> East
            old.sides[7], // West inv -> East inv
            old.sides[5], // South -> South inv
            old.sides[4], // South inv -> South
            old.sides[2], // East -> West
            old.sides[3], // East inv -> West inv
        ],
        insides: flip_vertical_insides(&old.insides),
        matching_tiles: [
            old.matching_tiles[1], // North -> North inv
            old.matching_tiles[0], // North inv -> North
            old.matching_tiles[6], // West -> East
            old.matching_tiles[7], // West inv -> East inv
            old.matching_tiles[5], // South -> South inv
            old.matching_tiles[4], // South inv -> South
            old.matching_tiles[2], // East -> West
            old.matching_tiles[3], // East inv -> West inv
        ],
    }
}

fn rotate_internals(tile: &TileInsides) -> TileInsides {
    let mut out = Box::new([false; (TILE_HEIGHT - 2) * (TILE_WIDTH - 2)]);
    out.chunks_exact_mut(TILE_WIDTH - 2)
        .enumerate()
        .for_each(|(x, row)| {
            row.iter_mut()
                .rev()
                .enumerate()
                .for_each(|(y, out)| *out = tile[(y * (TILE_WIDTH - 2)) + x])
        });
    out
}

pub fn rotate_cw(old: &Tile) -> Tile {
    Tile {
        sides: [
            old.sides[7], // West inv -> North
            old.sides[6], // West -> North inv
            old.sides[0], // North -> East
            old.sides[1], // North inv -> East inv
            old.sides[3], // East inv -> South
            old.sides[2], // East -> South inv
            old.sides[4], // North -> East
            old.sides[5], // North inv -> East inv
        ],
        insides: rotate_internals(&old.insides),
        matching_tiles: [
            old.matching_tiles[7], // West inv -> North
            old.matching_tiles[6], // West -> North inv
            old.matching_tiles[0], // North -> East
            old.matching_tiles[1], // North inv -> East inv
            old.matching_tiles[3], // East inv -> South
            old.matching_tiles[2], // East -> South inv
            old.matching_tiles[4], // South -> East
            old.matching_tiles[5], // South inv -> East inv
        ],
    }
}

pub fn rotate_image(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut out = image.clone();
    out.iter_mut().enumerate().for_each(|(x, row)| {
        row.iter_mut()
            .rev()
            .enumerate()
            .for_each(|(y, out)| *out = image[y][x])
    });
    out
}

pub fn flip_image_horizontal(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut out = image.clone();
    out.iter_mut()
        .zip(image.iter().rev())
        .for_each(|(dst, src)| dst.copy_from_slice(src));
    out
}

pub fn flip_image_vertical(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut out = image.clone();
    out.iter_mut().zip(image.iter()).for_each(|(dst, src)| {
        dst.iter_mut()
            .zip(src.iter().rev())
            .for_each(|(dst, src)| *dst = *src)
    });
    out
}
