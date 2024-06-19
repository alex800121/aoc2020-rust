use project_root::get_project_root;
use std::collections::BTreeMap;
use aoc2020::Coord;
type M<C> = BTreeMap<C, usize>;
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    dbg!([i64::max_value(), i64::min_value(), 3, 5].surround().len());
    dbg!([1, 1, 3, 5].adjacent().len());
}
