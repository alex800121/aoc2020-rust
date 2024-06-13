use itertools::Itertools;
use nom::{AsChar, InputTake};
use project_root::get_project_root;

const PASSPORT: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn valid_a(input: &str) -> bool {
    PASSPORT
        .iter()
        .all(|x| input.split_whitespace().map(|x| x.take(3)).contains(x))
}
fn valid_b(input: &str) -> bool {
    let c = input
        .split_whitespace()
        .filter_map(|x| {
            let (n, field) = x.split_once(':')?;
            let b = match n {
                "byr" => field
                    .parse::<u32>()
                    .ok()
                    .is_some_and(|y| (1920..=2002).contains(&y)),
                "iyr" => field
                    .parse::<u32>()
                    .ok()
                    .is_some_and(|y| (2010..=2020).contains(&y)),
                "eyr" => field
                    .parse::<u32>()
                    .ok()
                    .is_some_and(|y| (2020..=2030).contains(&y)),
                "hgt" => {
                    field
                        .strip_suffix("in")
                        .and_then(|x| x.parse::<u32>().ok())
                        .is_some_and(|y| (59..=76).contains(&y))
                        || field
                            .strip_suffix("cm")
                            .and_then(|x| x.parse::<u32>().ok())
                            .is_some_and(|y| (150..=193).contains(&y))
                }
                "hcl" => field
                    .strip_prefix('#')
                    .is_some_and(|s| s.len() == 6 && s.chars().all(|c| c.is_hex_digit())),
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .iter()
                    .any(|x| *x == field),
                "pid" => field.len() == 9 && field.chars().all(|c| c.is_ascii_digit()),
                _ => false,
            };
            if b {
                Some(n)
            } else {
                None
            }
        })
        .collect_vec();
    PASSPORT.iter().all(|x| c.contains(x))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.split("\n\n").collect_vec();
    println!("day4a: {}", &input.iter().filter(|x| valid_a(x)).count());
    println!("day4b: {}", &input.iter().filter(|x| valid_b(x)).count());
}
