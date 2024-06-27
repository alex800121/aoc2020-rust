use std::iter::from_fn;

use itertools::Itertools;

const INPUT: &str = "962713854";
const SMALL: usize = 9;
const BIG: usize = 1_000_000;
fn step(n: usize, v: &mut [usize]) {
    let current = v[0];
    let mut next = v[current];
    let next_3: [usize; 3] = std::array::from_fn(|_| {
        let b = next;
        next = v[b];
        b
    });
    let mut target = (current + n - 2).rem_euclid(n) + 1;
    loop {
        // dbg!(next_3, target);
        if !next_3.contains(&target) {
            break;
        }
        target = (target + n - 2).rem_euclid(n) + 1;
    }
    let target_next = v[target];
    // dbg!(current, target, target_next, next);
    v[0] = next;
    v[current] = next;
    v[target] = next_3[0];
    v[next_3[2]] = target_next;
    // dbg!(show(n, v));
}
fn build_input(m: usize) -> Vec<usize> {
    let n = m + 1;
    let input = INPUT
        .chars()
        .filter_map(|c| c.to_digit(10).map(|x| x as usize))
        .collect_vec();
    let mut i = 0;
    let mut output = Vec::from_iter(from_fn(|| {
        if i >= n {
            None
        } else {
            i += 1;
            Some(i)
        }
    }));
    output[m] = input[0];
    let mut i = 0;
    for a in input.iter() {
        output[i] = *a;
        i = *a;
    }
    if m <= input.len() {
        output[i] = input[0];
    } else {
        output[i] = input.len() + 1;
    }
    output
}
fn show(n: usize, v: &[usize]) -> String {
    let mut c = 1;
    let mut s = String::new();
    for _ in 1..n {
        c = v[c];
        s.push_str(&c.to_string());
    }
    s
}
pub fn run(_day: usize) {
    let mut small = build_input(SMALL);
    let mut big = build_input(BIG);
    // for _ in 0..10 {
    for _ in 0..100 {
        step(SMALL, &mut small);
    }
    println!("day23a: {}", show(SMALL, &small));
    for _ in 0..10_000_000 {
        step(BIG, &mut big);
    }
    println!("day23b: {}", big[1] * big[big[1]]);
}
