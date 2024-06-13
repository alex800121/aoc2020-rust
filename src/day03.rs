use itertools::Itertools;
use project_root::get_project_root;

// const W: usize = 11;
// const H: usize = 11;
const W: usize = 31;
const H: usize = 323;
type M = [[bool; W]; H];
type Index = (usize, usize);

fn slide((x, y): Index, m: &M) -> usize {
    m.iter()
        .enumerate()
        .filter(|(j, l)| {
            j % y == 0
                && ({
                    let i = (x * (j / y)) % W;
                    l[i]
                })
        })
        .count()
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input: M = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c == '#')
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    println!("day3a: {}", slide((3, 1), &input));
    println!(
        "day3b: {}",
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .map(|x| slide(x, &input))
            .iter()
            .product::<usize>()
    );
}
