use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;
use project_root::get_project_root;

fn read_players(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let (a, b) = input.split_once("\n\n").unwrap();
    (
        a.lines()
            .dropping(1)
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<VecDeque<_>>(),
        b.lines()
            .dropping(1)
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<VecDeque<_>>(),
    )
}
fn play_a(xs: &mut VecDeque<usize>, ys: &mut VecDeque<usize>) {
    loop {
        match (xs.pop_front(), ys.pop_front()) {
            (Some(x), Some(y)) => {
                if x > y {
                    xs.push_back(x);
                    xs.push_back(y);
                } else {
                    ys.push_back(y);
                    ys.push_back(x);
                }
            }
            (Some(x), None) => {
                xs.push_front(x);
                return;
            }
            (None, Some(y)) => {
                ys.push_front(y);
                return;
            }
            _ => {
                unreachable!()
            }
        }
    }
}
fn score(xs: &VecDeque<usize>) -> usize {
    xs.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * *x)
        .sum::<usize>()
}
fn play_b(
    xs: &mut VecDeque<usize>,
    ys: &mut VecDeque<usize>,
) -> bool {
    let mut played = BTreeSet::new();
    loop {
        let state = (xs.clone(), ys.clone());
        if played.contains(&state) {
            return true;
        }
        played.insert(state);
        match (xs.pop_front(), ys.pop_front()) {
            (Some(x), Some(y)) if xs.len() >= x && ys.len() >= y => {
                let mut new_xs = xs.clone();
                new_xs.truncate(x);
                let mut new_ys = ys.clone();
                new_ys.truncate(y);
                if play_b(&mut new_xs, &mut new_ys) {
                    xs.push_back(x);
                    xs.push_back(y);
                } else {
                    ys.push_back(y);
                    ys.push_back(x);
                }
            }
            (Some(x), Some(y)) => {
                if x > y {
                    xs.push_back(x);
                    xs.push_back(y);
                } else {
                    ys.push_back(y);
                    ys.push_back(x);
                }
            }
            (Some(x), None) => {
                xs.push_front(x);
                return true;
            }
            (None, Some(y)) => {
                ys.push_front(y);
                return false;
            }
            _ => {
                unreachable!()
            }
        }
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (mut xs, mut ys) = read_players(&input);
    let mut xs0 = xs.clone();
    let mut ys0 = ys.clone();
    play_a(&mut xs0, &mut ys0);
    println!("day22a: {}", [xs0, ys0].iter().map(score).sum::<usize>());
    play_b(&mut xs, &mut ys);
    println!("day22b: {}", [xs, ys].iter().map(score).sum::<usize>());
}
