use std::ops::{BitOr, RangeBounds};

use itertools::Itertools;
use nom::{bytes, AsBytes};
use num::PrimInt;
use project_root::get_project_root;

fn calc_votes(input: &[&[u8]]) -> (u32, u32) {
    let mut x = 0;
    let mut y = u32::max_value();
    for &l in input {
        let mut z: u32 = 0;
        for &c in l {
            z |= 1 << (c - b'a');
        }
        x |= z;
        y &= z;
    }
    (x.count_ones(), y.count_ones())
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .split("\n\n")
        .map(|b| b.lines().map(|l| l.as_bytes()).collect_vec())
        .collect_vec();
    let (a, b) =  input.iter().fold((0, 0), |(a, b), x| {
        let s = calc_votes(x);
        (a + s.0, b + s.1)
    });
    println!("day6a: {}", a);
    println!("day6b: {}", b);
}
