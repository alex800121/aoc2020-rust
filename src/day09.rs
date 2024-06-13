use std::cmp::Ordering;

use itertools::Itertools;
use project_root::get_project_root;

type Table = [[u64; 25]; 1000];

fn build_table(input: &[u64; 1000]) -> Table {
    let mut table = [[0; 25]; 1000];
    for n in 0..1000 {
        let x = input[n];
        table[n][0] = x;
        for i in 1..25 {
            if n >= i {
                table[n][i] = x + input[n - i];
            }
        }
    }
    table
}
fn solve_a(table: &Table) -> Option<u64> {
    'a: for i in 25..1000 {
        let x = table[i][0];
        for j in 1..25 {
            for k in 1..=j {
                if table[i - 25 + j][k] == x {
                    continue 'a;
                }
            }
        }
        return Some(x);
    }
    None
}
fn solve_b(target: u64, input: &[u64; 1000]) -> Option<u64> {
    use Ordering::*;
    let mut i = 0;
    let mut j = 1;
    let mut sum = input[0];
    while i < 1000 && j < 1000 {
        match sum.cmp(&target) {
            Equal => {
                return input[i..j]
                    .iter()
                    .minmax()
                    .into_option()
                    .map(|(x, y)| x + y);
            }
            Greater => {
                sum -= input[i];
                i += 1;
            }
            Less => {
                sum += input[j];
                j += 1;
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
    let input: [u64; 1000] = input
        .lines()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect_vec()
        .try_into()
        .unwrap();
    let a = solve_a(&build_table(&input)).unwrap();
    println!("day9a: {}", a);
    println!("day9b: {}", solve_b(a, &input).unwrap());
}
