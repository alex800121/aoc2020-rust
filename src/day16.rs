use std::ops::{RangeBounds, RangeInclusive};

use itertools::Itertools;
use project_root::get_project_root;

type RefRange<'a> = [(&'a str, [RangeInclusive<u32>; 2]); 20];
type Ticket = [u32; 20];
fn parse_input(input: &str) -> Option<(RefRange, Ticket, Vec<Ticket>)> {
    let mut input = input.split("\n\n");
    let ref_range: RefRange = input
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
fn scan(r: &RefRange, ts: &[Ticket]) -> u32 {
    let mut acc = 0;
    for t in ts {
        for p in t {
            if !r.iter().any(|x| x.1.iter().any(|y| y.contains(p))) {
                acc += *p;
            }
        }
    }
    acc
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (ref_range, ticket0, tickets) = parse_input(&input).unwrap();
    dbg!(scan(&ref_range, &tickets));
}
