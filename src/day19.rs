use itertools::Itertools;
use nom::AsChar;
use project_root::get_project_root;

type Rules = Vec<Rule>;
#[derive(Debug)]
enum Rule {
    Or(Vec<Vec<usize>>),
    Ch(char),
}
fn parse_input(input: &str) -> Option<(Rules, Vec<&str>)> {
    let (a, b) = input.split_once("\n\n")?;
    Some((parse_rules(a), b.lines().collect_vec()))
}
fn parse_seq<'a>(input: &'a str, rules: &Rules, xs: &Vec<usize>) -> Vec<&'a str> {
    use Rule::*;
    let mut input = vec![input];
    for x in xs {
        let mut new_input = vec![];
        for i in input.drain(..) {
            match &rules[*x] {
                Or(ys) => {
                    for y in ys {
                        let j = parse_seq(i, rules, y);
                        new_input.extend(j);
                    }
                }
                Ch(c) => {
                    if i.starts_with(*c) {
                        new_input.push(&i[1..]);
                    }
                }
            }
        }
        input = new_input;
    }
    input
}
fn parse_rules(input: &str) -> Rules {
    use Rule::*;
    let mut n = 0;
    let mut output = vec![];
    for (a, b) in input.trim().lines().filter_map(|x| x.split_once(": ")) {
        let a = a.parse::<usize>().unwrap();
        if n <= a {
            n = a;
            output.resize_with(a + 1, || Ch(' '));
        }
        match b.trim_matches('"') {
            b if b.chars().next().is_some_and(|c| c.is_alpha()) => {
                output[a] = Ch(b.chars().next().unwrap());
            }
            b => {
                let b = b.split('|');
                output[a] = Or(b
                    .map(|x| {
                        x.split_whitespace()
                            .map(|y| y.parse::<usize>().unwrap())
                            .collect_vec()
                    })
                    .collect_vec());
            }
        }
    }
    output
}
pub fn run(day: usize) {
    use Rule::*;
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (mut rules, input) = parse_input(&input).unwrap();
    println!(
        "day19a: {}",
        input
            .iter()
            .filter(|x| parse_seq(x, &rules, &vec![0]).contains(&""))
            .count()
    );
    rules[8] = Or(vec![vec![42], vec![42, 8]]);
    rules[11] = Or(vec![vec![42, 31], vec![42, 11, 31]]);
    println!(
        "day19b: {}",
        input
            .iter()
            .filter(|x| parse_seq(x, &rules, &vec![0]).contains(&""))
            .count()
    );
}
