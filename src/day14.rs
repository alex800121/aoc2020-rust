use itertools::Itertools;
use num::PrimInt;
use project_root::get_project_root;
use std::collections::BTreeMap;
use std::u64;

fn solve_a(input: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    let mut c = BTreeMap::new();
    for (i, v) in input {
        let mut i0: u64 = 0;
        let mut i1: u64 = u64::max_value() - (2u64.pow(36) - 1);
        for (n, c) in i.bytes().rev().enumerate() {
            match c {
                b'1' => i0 |= 1 << n,
                b'0' => i1 |= 1 << n,
                _ => {}
            }
        }
        i1 = !i1;
        for (n, x) in v {
            let x = (x | i0) & i1;
            c.insert(*n, x);
        }
    }
    c.values().sum::<u64>()
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .split("mask = ")
        .filter_map(|b| {
            let mut b = b.lines();
            let a = b.next()?;
            let mut v = Vec::new();
            for l in b {
                let x = l.trim_start_matches("mem[");
                let (a, b) = x.split_once("] = ")?;
                v.push((a.parse::<u64>().ok()?, b.parse::<u64>().ok()?));
            }
            Some((a, v))
        })
        .collect_vec();
    println!("day14a: {}", solve_a(&input));
}
