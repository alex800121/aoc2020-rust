use std::ops::RangeInclusive;

use itertools::Itertools;
use project_root::get_project_root;

type RefRange<'a, const N: usize> = [(&'a str, [RangeInclusive<u32>; 2]); N];
type Ticket = [u32; N];
const N: usize = 20;
fn parse_input(input: &str) -> Option<(RefRange<N>, Ticket, Vec<Ticket>)> {
    let mut input = input.split("\n\n");
    let ref_range: RefRange<N> = input
        .next()?
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(": ")?;
            let b: [RangeInclusive<u32>; 2] = b
                .split(" or ")
                .filter_map(|x| {
                    let (x, y) = x.split_once('-')?;
                    Some(x.parse::<u32>().ok()?..=y.parse::<u32>().ok()?)
                })
                .collect_vec()
                .try_into()
                .ok()?;
            Some((a, b))
        })
        .collect_vec()
        .try_into()
        .ok()?;
    let ticket0: Ticket = input
        .next()?
        .trim_start_matches("your ticket:")
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect_vec()
        .try_into()
        .ok()?;
    let tickets: Vec<Ticket> = input
        .next()?
        .trim_start_matches("nearby tickets:")
        .trim()
        .lines()
        .filter_map(|l| {
            l.trim()
                .split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect_vec()
                .try_into()
                .ok()
        })
        .collect_vec();
    Some((ref_range, ticket0, tickets))
}
fn scan<'a>(r: &'a RefRange<N>, ts: &'a [Ticket]) -> (Vec<Ticket>, u32) {
    let mut acc = 0;
    let mut v = Vec::from(ts);
    let mut i = 0;
    while i < v.len() {
        let t = v[i];
        let mut b = false;
        for p in t {
            if !r.iter().any(|x| x.1.iter().any(|y| y.contains(&p))) {
                acc += p;
                b = true;
            }
        }
        if b {
            v.remove(i);
        } else {
            i += 1;
        }
    }
    (v, acc)
}
fn solve_b<'a>(ref_range: &'a RefRange<N>, tickets: &[Ticket]) -> [&'a str; N] {
    let mut output = [""; N];
    let mut tmp: [u32; N] = [(1 << N) - 1; N];
    for ts in tickets {
        for (i, t) in ts.iter().enumerate() {
            let mut k = 0;
            for (j, x) in ref_range.iter().enumerate() {
                if x.1.iter().any(|y| y.contains(t)) {
                    k |= 1 << j
                }
            }
            tmp[i] &= k;
        }
    }
    let mut start = *tmp.iter().find(|x| x.count_ones() == 1).unwrap();
    let mut visited = 0u32;
    while visited != (1 << N) - 1 {
        let mut next_start = 0;
        for t in tmp.iter_mut() {
            if *t != start && 0 != *t & start {
                *t &= !start;
                if t.count_ones() == 1 {
                    next_start = *t;
                }
            }
        }
        visited |= start;
        start = next_start;
    }
    for (i, t) in tmp.iter().map(|x| x.trailing_zeros()).enumerate() {
        output[i] = ref_range[t as usize].0;
    }
    output
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (ref_range, ticket0, tickets) = parse_input(&input).unwrap();
    let (tickets, a) = scan(&ref_range, &tickets);
    println!("day16a: {}", a);
    println!(
        "day16b: {}",
        solve_b(&ref_range, &tickets)
            .iter()
            .enumerate()
            .map(|(i, x)| if x.starts_with("departure") {
                ticket0[i] as u64
            } else {
                1
            })
            .product::<u64>()
    );
}
