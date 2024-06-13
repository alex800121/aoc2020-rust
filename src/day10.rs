use itertools::Itertools;
use project_root::get_project_root;

fn solve_a(input: &[u32]) -> Option<u32> {
    let mut input = input.iter();
    let mut a = 0;
    let mut c = 0;
    let mut x = input.next()?;
    for y in input {
        match y - x {
            1 => {
                a += 1;
            }
            3 => {
                c += 1;
            }
            _ => {}
        }
        x = y;
    }
    Some(c * a)
}
fn solve_b(input: &[u32]) -> Option<u64> {
    let target = input.last()?;
    let mut v = vec![0; *target as usize + 1];
    v[0] = 1;
    let l = v.len();
    for i in 1..l {
        if input.contains(&(i as u32)) {
            for j in 1..=3 {
                if i >= j {
                    v[i] += v[i - j];
                }
            }
        }
        // dbg!(v[i]);
    }
    v.last().copied()
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
        .filter_map(|l| l.parse::<u32>().ok())
        .collect_vec();
    input.sort();
    input.insert(0, 0);
    let a = input.last().unwrap();
    input.push(a + 3);
    println!("day10a: {}", solve_a(&input).unwrap());
    // dbg!(u64::max_value());
    println!("day10b: {}", solve_b(&input).unwrap());
}
