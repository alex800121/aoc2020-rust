use aoc2020::Coord;
use itertools::Itertools;
use project_root::get_project_root;
use std::collections::{BTreeMap, BTreeSet};

// HexCoord
//  nw  ne
//    ##
// w ### e
//   ##
// sw  se

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct HexCoord([i64; 2]);

const REF: [(&str, HexCoord); 6] = [
    ("e", HexCoord([1, 0])),
    ("se", HexCoord([0, -1])),
    ("sw", HexCoord([-1, -1])),
    ("w", HexCoord([-1, 0])),
    ("nw", HexCoord([0, 1])),
    ("ne", HexCoord([1, 1])),
];
impl Coord for HexCoord {
    fn adjacent(&self) -> Vec<Box<Self>> {
        self.surround()
    }

    fn surround(&self) -> Vec<Box<Self>> {
        REF.iter()
            .map(|x| Box::new(HexCoord([x.1 .0[0] + self.0[0], x.1 .0[1] + self.0[1]])))
            .collect_vec()
    }
}
fn flip_tile(input: &str) -> Option<HexCoord> {
    let mut input = input.trim();
    let mut output = HexCoord([0, 0]);
    while !input.is_empty() {
        let (pat, coord) = REF.iter().find(|(pat, _)| input.starts_with(pat))?;
        input = &input[(pat.len())..];
        output.0[0] += coord.0[0];
        output.0[1] += coord.0[1];
    }
    Some(output)
}
fn step(tiles: &mut BTreeSet<HexCoord>) {
    let mut count = BTreeMap::new();
    for HexCoord([x0, y0]) in tiles.iter() {
        for (_, HexCoord([x, y])) in REF {
            count
                .entry(HexCoord([x0 + x, y0 + y]))
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }
    }
    let mut output = BTreeSet::new();
    for i in tiles.iter() {
        if count.get(i).is_some_and(|x| [1, 2].contains(x)) {
            output.insert(*i);
        }
    }
    for (i, c) in count.iter() {
        if *c == 2 && !tiles.contains(i) {
            output.insert(*i);
        }
    }
    *tiles = output;
}
fn flip_all(input: &str) -> Option<BTreeSet<HexCoord>> {
    let mut output = BTreeSet::new();
    for input in input.lines() {
        let coord = flip_tile(input)?;
        if !output.remove(&coord) {
            output.insert(coord);
        }
    }
    Some(output)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut a = flip_all(&input).unwrap();
    println!("day24a: {}", a.len());
    for _ in 0..100 {
        step(&mut a); 
    }
    println!("day24b: {}", a.len());
}
