use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let mut bus_routes: Vec<(u64, u64)> = input_file
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(off, br)| {
            if br != "x" {
                Some((br.parse::<u64>().unwrap(), off as u64))
            } else {
                None
            }
        })
        .collect();

    let big_modulo = bus_routes.iter().fold(1, |acc, br| acc * br.0);
    let max_offset = bus_routes[bus_routes.len() - 1].1;
    let bus_routes: Vec<BusRoute> = bus_routes
        .iter_mut()
        .map(|br| BusRoute::new(br.0, max_offset - br.1, big_modulo))
        .collect();
    let ans = bus_routes.iter().fold(0, |acc, br| acc + br.get_product()) % big_modulo - max_offset;

    println!("Answer: {}", ans);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
struct BusRoute {
    route_num: u64,
    offset: u64,
    moduli: u64,
    inverse: u64,
}

impl BusRoute {
    fn new(route_num: u64, offset: u64, big_modulo: u64) -> BusRoute {
        let moduli = big_modulo / route_num;
        let inverse = modular_inverse(moduli, route_num);
        BusRoute {
            route_num,
            offset,
            moduli,
            inverse,
        }
    }

    fn get_product(&self) -> u64 {
        self.offset * self.moduli * self.inverse
    }
}

fn modular_inverse(num: u64, modulo: u64) -> u64 {
    let simplified = num % modulo;
    let mut inverse = 1;
    while inverse * simplified % modulo != 1 {
        inverse += 1;
    }

    inverse
}
