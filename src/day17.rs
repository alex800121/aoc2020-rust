use aoc2020::Coord;
use project_root::get_project_root;
use std::collections::{BTreeMap, BTreeSet};
type S<const N: usize> = BTreeSet<[i64; N]>;
fn next_cubes<const N: usize>(m: &mut S<N>) {
    let mut acc = BTreeMap::new();
    for &i in m.iter() {
        for j in i.surround() {
            acc.entry(*j)
                .and_modify(|y: &mut usize| {
                    *y += 1;
                })
                .or_insert(1);
        }
    }
    let mut next = BTreeSet::new();
    for (i, x) in acc.iter() {
        if !m.contains(i) && *x == 3 {
            next.insert(*i);
        }
    }
    m.retain(|i| acc.get(i).is_some_and(|y| *y == 2 || *y == 3));
    m.extend(next);
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut m3: S<3> = BTreeSet::from_iter(input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some([x as i64, y as i64, 0])
            } else {
                None
            }
        })
    }));
    for _ in 0..6 {
        next_cubes(&mut m3);
    }
    let mut m4: S<4> = BTreeSet::from_iter(input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some([x as i64, y as i64, 0, 0])
            } else {
                None
            }
        })
    }));
    for _ in 0..6 {
        next_cubes(&mut m4);
    }
    println!("day17a: {}", m3.len());
    println!("day17b: {}", m4.len());
}
