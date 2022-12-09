use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let count = fs::read_to_string(FILENAME)
        .expect("Some error with file")
        .lines()
        .filter(|line| PwdTest::new(line).is_okay())
        .count();

    println!("{} passwords meet the requirements", count);
}

struct PwdTest<'a> {
    pass: &'a str,
    c: char,
    min: u32,
    max: u32,
}

impl<'a> PwdTest<'a> {
    fn new(line: &str) -> PwdTest {
        let v: Vec<&str> = line
            .split(|delim| delim == '-' || delim == ':' || delim == ' ')
            .filter(|s| !s.is_empty())
            .collect();

        let pass = v[3];
        let c = v[2].parse().expect("could not parse to char");
        let min = v[0].parse().expect("could not parse to u8");
        let max = v[1].parse().expect("could not parse to u8");

        PwdTest { pass, c, min, max }
    }

    fn is_okay(&self) -> bool {
        let count = self.pass.matches(self.c).count() as u32;
        if count >= self.min && count <= self.max {
            return true;
        }

        false
    }
}
