const DOOR: u64 = 1526110;
const CARD: u64 = 20175123;
const DIVIDER: u64 = 20201227;
const SUBJECT: u64 = 7;
fn decrypt(subject: u64, divider: u64, card: u64, door: u64) -> u64 {
    let mut loop_size = 0;
    let mut v = 1;
    while v != card {
        v = (v * subject) % divider;
        loop_size += 1;
    }
    let mut v = 1;
    for _ in 0..loop_size {
        v = (v * door) % divider;
    }
    v
}
pub fn run(_day: usize) {
    println!("day25a: {}", decrypt(SUBJECT, DIVIDER, CARD, DOOR));
    println!("Merry Chistmas!");
}
