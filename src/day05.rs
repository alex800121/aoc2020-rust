use itertools::Itertools;
use project_root::get_project_root;

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let mut input = input.lines().map(|l| {
        l.chars().fold(0, |acc, c| match c {
            'B' | 'R' => acc * 2 + 1,
            _ => acc * 2,
        })
    }).collect_vec();
    input.sort();
    println!("day5a: {}", input.iter().max().unwrap());
    let l = input.len();
    let mut i = 0;
    while i < l - 1 {
        if input[i] == input[i + 1] - 2 {
            break;
        }
        i += 1;
    }
    println!("day5b: {}", input[i] + 1);
}
