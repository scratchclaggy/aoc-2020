const PRIME: u64 = 7;
const MOD: u64 = 20201227;
const CARD: u64 = 16915772;
const DOOR: u64 = 18447943;

fn main() {
    let mut card = Key::new(CARD);
    let mut door = Key::new(DOOR);
    let mut memo = vec![];

    memo.push(PRIME);
    card.crack(&mut memo);
    door.crack(&mut memo);

    println!("Card -> pub: {}, prv: {}", card.public, card.private);
    println!("Door -> pub: {}, prv: {}", door.public, door.private);
    println!("Handshake: {}", door.handshake(card.public));
}

struct Key {
    public: u64,
    private: usize,
}

impl Key {
    fn new(public: u64) -> Key {
        Key { public, private: 0 }
    }

    fn crack(&mut self, memo: &mut Vec<u64>) {
        if let Some(private_key) = memo.iter().position(|x| *x == self.public) {
            self.private = private_key;
        } else {
            let mut intermediate_val = *memo.last().unwrap();
            while intermediate_val != self.public {
                intermediate_val = key_gen(intermediate_val, PRIME);
                memo.push(intermediate_val);
            }
            self.private = memo.len();
        }
    }

    fn handshake(&self, other_pub: u64) -> u64 {
        let mut handshake = 1;
        for _ in 0..self.private {
            handshake = key_gen(handshake, other_pub);
        }

        handshake
    }
}

fn key_gen(mut input: u64, prime: u64) -> u64 {
    input *= prime;
    input %= MOD;
    input
}
