use itertools::Itertools;
use project_root::get_project_root;

fn valid_a((a, b, c, pass): &(u32, u32, char, &str)) -> bool {
    let l = pass.chars().filter(|x| x == c).count() as u32;
    l >= *a && l <= *b
}
fn valid_b((a, b, c, pass): &(u32, u32, char, &str)) -> bool {
    (|| {
        let c0 = pass.chars().nth(*a as usize - 1)?;
        let c1 = pass.chars().nth(*b as usize - 1)?;
        Some((c0 == *c || c1 == *c) && c0 != c1)
    })()
    .unwrap_or(false)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once('-')?;
            let (b, c) = b.split_once(' ')?;
            let (c, pass) = c.split_once(": ")?;
            Some((
                a.parse::<u32>().ok()?,
                b.parse::<u32>().ok()?,
                c.chars().next()?,
                pass,
            ))
        })
        .collect_vec();
    println!("day2a: {}", input.iter().filter(|x| valid_a(x)).count());
    println!("day2b: {}", input.iter().filter(|x| valid_b(x)).count());
}
