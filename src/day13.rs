use aoc2020::chinese_rem;
use itertools::Itertools;
use project_root::get_project_root;

pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (a, bus) = input.split_once('\n').unwrap();
    let a = a.parse::<i64>().unwrap();
    let bus = bus
        .trim_end()
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            let s = s.parse::<i64>().ok()?;
            Some((s, -(i as i64)))
        })
        .collect_vec();
    println!(
        "day13a: {}",
        bus.iter()
            .map(|(y, _)| (y, (-a).rem_euclid(*y)))
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .next()
            .map(|x| x.0 * x.1)
            .unwrap()
    );
    println!("day13b: {}", chinese_rem(&bus).unwrap());
}
