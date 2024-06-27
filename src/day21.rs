use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use project_root::get_project_root;

type Recipe<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;
type Ingredients<'a> = BTreeMap<&'a str, usize>;
fn read_recipe(input: &str) -> Option<(Ingredients<'_>, Recipe<'_>)> {
    let mut output = BTreeMap::new();
    let mut ingredients = BTreeMap::new();
    for l in input.lines() {
        let (a, b) = l.trim_end_matches(')').split_once(" (contains ")?;
        for ingredient in a.split_whitespace() {
            ingredients
                .entry(ingredient)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }
        for allergen in b.split(", ") {
            output
                .entry(allergen)
                .and_modify(|ing: &mut BTreeSet<&str>| {
                    let a: BTreeSet<&str> = a.split_whitespace().collect();
                    *ing = ing.intersection(&a).copied().collect::<BTreeSet<_>>();
                })
                .or_insert(a.split_whitespace().collect::<BTreeSet<_>>());
        }
    }
    for xs in output.values() {
        for x in xs.iter() {
            ingredients.remove(x);
        }
    }
    Some((ingredients, output))
}
fn solve_b(mut recipe: Recipe) -> Option<String> {
    let mut v = vec![];
    while !recipe.is_empty() {
        let (allergen, ingredient) = recipe.iter().find_map(|(allergen, ingredients)| {
            if ingredients.len() == 1 {
                Some((*allergen, *ingredients.iter().next()?))
            } else {
                None
            }
        })?;
        v.push((allergen, ingredient));
        recipe.remove(allergen);
        recipe.values_mut().for_each(|v| {
            v.remove(ingredient);
        });
    }
    v.sort_by(|a, b| a.0.cmp(b.0));
    Some(v.iter().map(|x| x.1).join(","))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let (ingredients, recipe) = read_recipe(&input).unwrap();
    println!("day21a: {}", ingredients.values().sum::<usize>());
    println!("day21b: {}", solve_b(recipe).unwrap());
}
