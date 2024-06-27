use std::collections::{BTreeMap, BTreeSet};
use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, space1, u32},
    combinator::{map, opt, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use project_root::get_project_root;

fn bag(input: &str) -> IResult<&str, &str> {
    let (input, x) = take_until(" bag")(input)?;
    let (input, _) = tuple((tag(" bag"), opt(char('s'))))(input)?;
    Ok((input, x))
}
fn inner_bag(input: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(u32, space1, bag)(input)
}
fn no_bag(input: &str) -> IResult<&str, BTreeMap<&str, u32>> {
    value(BTreeMap::new(), tag("no other bags"))(input)
}
fn inner_bags(input: &str) -> IResult<&str, BTreeMap<&str, u32>> {
    terminated(
        alt((
            value(BTreeMap::new(), no_bag),
            map(separated_list1(tag(", "), inner_bag), |l| {
                BTreeMap::from_iter(l.into_iter().map(|(x, y)| (y, x)))
            }),
        )),
        char('.'),
    )(input)
}
fn parse_bag(input: &str) -> IResult<&str, (&str, BTreeMap<&str, u32>)> {
    let (input, b) = bag(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, s) = alt((no_bag, inner_bags))(input)?;
    Ok((input, (b, s)))
}
type Bags<'a> = BTreeMap<&'a str, BTreeMap<&'a str, u32>>;
fn solve_a(b: &Bags<'_>) -> usize {
    let mut acc = BTreeSet::new();
    let mut start = BTreeSet::from(["shiny gold"]);
    while !start.is_empty() {
        let mut start0 = BTreeSet::new();
        for i in start.iter() {
            start0.extend(BTreeSet::from_iter(b.iter().filter_map(|(k, x)| {
                if x.contains_key(i) {
                    Some(k)
                } else {
                    None
                }
            })));
        }
        acc.extend(start0.clone());
        start = start0;
    }
    acc.len()
}
fn solve_b(b: &Bags) -> u32 {
    let mut start = BTreeMap::from([("shiny gold", 1)]);
    let mut acc = BTreeMap::new();
    while !start.is_empty() {
        let mut start0 = BTreeMap::new();
        for (name, n) in start.iter() {
            if let Some(inner) = b.get(name) {
                for (&inner_name, &inner_n) in inner.iter() {
                    if let Some(x) = start0.get_mut(inner_name) {
                        *x += n * inner_n;
                    } else {
                        start0.insert(inner_name, n * inner_n);
                    }
                }
            }
        }
        start = start0.clone();
        for (name, n) in start0.into_iter() {
            if let Some(v) = acc.get_mut(name) {
                *v += n;
            } else {
                acc.insert(name, n);
            }
        }
    }
    acc.values().sum()
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
        .filter_map(|x| parse_bag(x).ok().map(|x| x.1))
        .collect::<Bags>();
    println!("day7a: {}", solve_a(&input));
    println!("day7b: {}", solve_b(&input));
}
