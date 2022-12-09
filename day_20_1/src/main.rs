use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::default::Default;

const TILE_WIDTH: usize = 10;
const TILE_HEIGHT: usize = 10;
type Tile = [bool; TILE_WIDTH * TILE_HEIGHT];

const FILENAME: &str = "input.txt";


fn extract_tiles(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<(usize, TileSides)> {
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
        let tile = TileSides::new(&tile);

        tiles.push((tile_num, tile));
        if lines.next().is_none() {
            break;
        }
    }

    tiles
}

fn print_tile(tile: &Tile){
    for row in tile.chunks_exact(TILE_WIDTH){
        for &bit in row{
            if bit{
                print!("#");
            }else{
                print!(".");
            }
        }
        println!("");
    }
}

struct TileSides {
    sides: [u16; 8],   
    matching: [HashSet<usize>; 8],
}

impl TileSides{
    fn north(&self)->u16{
        self.sides[0]
    }
    fn north_inverse(&self)->u16{
        self.sides[1]
    }
    fn east(&self)->u16{
        self.sides[2]
    }
    fn east_inverse(&self)->u16{
        self.sides[3]
    }
    fn south(&self)->u16{
        self.sides[4]
    }
    fn south_inverse(&self)->u16{
        self.sides[5]
    }
    fn west(&self)->u16{
        self.sides[6]
    }
    fn west_inverse(&self)->u16{
        self.sides[7]
    }
}

impl TileSides {
    fn new(bool_string: &Tile) -> TileSides {
        let mut sides = [0; 8];
        let mut north: u16 = 0; 
        

        // North border
        for i in 0..TILE_WIDTH {
            north <<= 1;
            if bool_string[i] {
                north |= 1;
            }
        }
        let north_inverse = north.reverse_bits() >> 6;
        
        let mut east = 0u16;
        let mut west = 0u16;
        for row in bool_string.chunks_exact(10){
            east <<= 1;
            west <<= 1;
            west |= row[0] as u16;
            east |= row[TILE_WIDTH-1] as u16; 
        }
        let east_inverse = east.reverse_bits() >> 6;
        let west_inverse = west.reverse_bits() >> 6;

        let bottom_row = &bool_string[(TILE_HEIGHT-1) * TILE_WIDTH..];
        let south = bottom_row.iter().fold(0u16, |acc, &arg| (acc << 1) | arg as u16);
        let south_inverse = south.reverse_bits() >> 6;
        
        TileSides {sides: [north, north_inverse, east, east_inverse, south, south_inverse, west, west_inverse], matching: Default::default()}
        
    }
}



fn main() {
    let file = File::open(FILENAME).map(BufReader::new).expect("File I/O Error");
    let mut tiles = extract_tiles(file.lines().map(|s| s.unwrap()));

    let mut tile_map: HashMap::<usize, TileSides> = HashMap::new();

    while let Some((id, mut tile)) = tiles.pop(){
        for (other_id, other_tile) in tiles.iter_mut() {
            // For each side
            for i in 0..8 {
               // For each side in other tile
               for j in 0..8 {
                    if tile.sides[i] == other_tile.sides[j] {
                        tile.matching[i].insert(*other_id);
                        other_tile.matching[j].insert(id);
                    }
               } 
            }
        }
        tile_map.insert(id, tile);
    }

    let ans = tile_map.iter().filter(|(id, tile)| tile.matching.iter().filter(
        |side| side.is_empty()).count() == 4
    )
    .map(|(id, _)| id)
    .product::<usize>();

    println!("{}", ans);
}


#[cfg(test)]
mod tests{
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
    fn test(){
        let mut tiles = extract_tiles(INPUT);

        let mut tile_map: HashMap::<usize, TileSides> = HashMap::new();

        while let Some((id, mut tile)) = tiles.pop(){
            for (other_id, other_tile) in tiles.iter_mut() {
                // For each side
                for i in 0..8 {
                // For each side in other tile
                for j in 0..8 {
                        if tile.sides[i] == other_tile.sides[j] {
                            tile.matching[i].insert(*other_id);
                            other_tile.matching[j].insert(id);
                        }
                } 
                }
            }
            tile_map.insert(id, tile);
        }

        for (id, tile) in tile_map.iter() {
            println!("{}: ", id);
            for side in tile.matching.iter() {
                print!(" ");
                for matched_side in side.iter() {
                    print!(" {}", matched_side);
                }
                println!();
            }
        }
        panic!();
    }
    
}