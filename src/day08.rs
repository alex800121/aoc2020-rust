use std::collections::BTreeSet;

use itertools::Itertools;
use project_root::get_project_root;

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum Op {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}
fn parse_op(input: &str) -> Option<Op> {
    use Op::*;
    let (a, b) = input.split_once(' ')?;
    let b = b.trim_matches('+').parse::<i32>().ok()?;
    match a {
        "acc" => Some(Acc(b)),
        "nop" => Some(Nop(b)),
        "jmp" => Some(Jmp(b)),
        _ => None,
    }
}
fn op(o: &Op, pos: &mut usize, x: &mut i32) {
    use Op::*;
    match o {
        Acc(i) => {
            *x += i;
            *pos += 1;
        }
        Nop(_) => {
            *pos += 1;
        }
        Jmp(i) => {
            *pos = (*pos as i32 + i) as usize;
        }
    }
}
fn solve_a(ins: &[Op], output: &mut i32, pos: &mut usize) -> Option<i32> {
    let mut past_pos = BTreeSet::new();
    while !past_pos.contains(pos) {
        past_pos.insert(*pos);
        op(ins.get(*pos)?, pos, output);
    }
    Some(*output)
}
fn solve_b(ins: &mut [Op]) -> Option<i32> {
    use Op::*;
    let l = ins.len();
    for i in 0..l {
        let o = ins.get_mut(i)?;
        match o {
            Nop(x) => {
                *o = Jmp(*x);
            },
            Jmp(x) => {
                *o = Nop(*x);
            },
            _ => {
                continue;
            }
        }
        let mut output = 0;
        if solve_a(ins, &mut output, &mut 0).is_none() {
            return Some(output);
        }
        let o = ins.get_mut(i)?;
        match o {
            Nop(x) => {
                *o = Jmp(*x);
            },
            Jmp(x) => {
                *o = Nop(*x);
            },
            _ => {
                continue;
            }
        }
    }
    None
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut input = input.lines().filter_map(parse_op).collect_vec();
    println!("day8a: {}", solve_a(&input, &mut 0, &mut 0).unwrap());
    println!("day8b: {}", solve_b(&mut input).unwrap());
}
