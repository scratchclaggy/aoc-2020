// My thanks to /u/Lakret for his demonstration of this technique of using array elements to point
// to indexes

const TOTAL_CUPS: usize = 1_000_000;
const ITERATIONS: usize = 10_000_000;

fn main() {
    let input = "364297581";
    let mut input = input.chars().map(|num| num.to_digit(10).unwrap() as usize);
    let mut cups = vec![];
    cups.resize(TOTAL_CUPS + 1, 0);
    let mut prev = input.next().unwrap();
    cups[0] = prev;
    while let Some(next) = input.next() {
        cups[prev] = next;
        prev = next;
    }
    cups[prev] = 10;

    for i in 11..TOTAL_CUPS + 1 {
        cups[i - 1] = i;
    }

    cups[TOTAL_CUPS] = cups[0];

    for _ in 0..ITERATIONS {
        crab_cups(&mut cups);
    }

    let x = cups[1];
    let y = cups[x];

    println!("Ans: {} * {} = {}", x, y, x * y);
}

fn crab_cups(cups: &mut [usize]) {
    let current = cups[0];
    let p1 = cups[current];
    let p2 = cups[p1];
    let p3 = cups[p2];
    let mut dst = current;

    while dst == current || dst == p1 || dst == p2 || dst == p3 || dst == 0 {
        dst -= 1;
        if dst == 0 {
            dst = TOTAL_CUPS;
        }
    }

    // The current cup needs to point to what is after the third pick
    cups[current] = cups[p3];

    // The destination needs to point to the first picked up, and the third picked up needs to
    // point to whatever the destination was previously pointing to
    let dst_next = cups[dst];
    cups[dst] = p1;
    cups[p3] = dst_next;

    // Next up is whatever current is pointing to now
    // println!(
    //     "current = {} | p1, p2, p3 = {}, {}, {} | dst, dst_next = {}, {}",
    //     current, p1, p2, p3, dst, dst_next
    // );
    cups[0] = cups[current];
}
