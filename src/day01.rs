use itertools::Itertools;
use project_root::get_project_root;
use std::cmp::Ordering::*;

fn solve(n: usize, target: i32, input: &[i32]) -> Option<i32> {
    let mut i = 0;
    let mut v: Vec<(usize, i32)> = Vec::new();
    let mut acc = target;
    let mut n = n;
    loop {
        match n.cmp(&0) {
            Equal => match acc.cmp(&0) {
                Equal => {
                    return Some(v.iter().map(|x| x.1).product());
                }
                Less => {
                    let (j, x) = v.pop()?;
                    i = j + 1;
                    n += 1;
                    acc += x;
                }
                Greater => {
                    let (_, x) = v.pop()?;
                    acc += x;
                    let (j, x) = v.pop()?;
                    i = j + 1;
                    n += 2;
                    acc += x;
                }
            },
            _ => match acc.cmp(&0) {
                Equal | Less => {
                    let (j, x) = v.pop()?;
                    i = j + 1;
                    n += 1;
                    acc += x;
                }
                Greater => {
                    if let Some(x) = input.get(i) {
                        v.push((i, *x));
                        acc -= x;
                        n -= 1;
                        i += 1;
                    } else {
                        let (j, x) = v.pop()?;
                        i = j + 1;
                        n += 1;
                        acc += x;
                    }
                }
            },
        }
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut input = input
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect_vec();
    input.sort();
    input.reverse();
    println!("day1a: {}", solve(2, 2020, &input[..]).unwrap());
    println!("day1b: {}", solve(3, 2020, &input[..]).unwrap());
}
