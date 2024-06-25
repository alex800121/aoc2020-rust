use aoc2020::{Clockwise, Coord, Transpose};
use itertools::Itertools;
use project_root::get_project_root;
use std::collections::BTreeMap;

const N: usize = 10;
type Tile = [[bool; N]; N];
type NumTile = (u64, Tile);
type Index = [i64; 2];

fn parse_numtile(input: &str) -> Option<NumTile> {
    let input = input.trim_start_matches("Tile ");
    let (no, tile) = input.split_once(":\n")?;
    let tile = tile
        .lines()
        .filter_map(|x| x.chars().map(|y| y == '#').collect_vec().try_into().ok())
        .collect_vec()
        .try_into()
        .ok()?;
    Some((no.parse::<u64>().ok()?, tile))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.split("\n\n").filter_map(parse_numtile).collect_vec();
    dbg!(input);
}
