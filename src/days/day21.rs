use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve_a(
    recipes: &[HashSet<&str>],
    ingredients: &HashSet<&str>,
    allergens: &HashMap<&str, HashSet<&str>>,
) -> usize {
    let cannot: HashSet<&str> = ingredients
        .difference(
            &allergens
                .values()
                .flatten()
                .copied()
                .collect::<HashSet<&str>>(),
        )
        .copied()
        .collect();

    recipes
        .iter()
        .map(|ing| ing.intersection(&cannot).count())
        .sum()
}

fn solve_b(mut allergens: HashMap<&str, HashSet<&str>>) -> String {
    let mut concluded: HashMap<&str, &str> = HashMap::new();

    while !allergens.is_empty() {
        let unique: &str = allergens
            .iter()
            .find(|(_, ingr)| ingr.len() == 1)
            .map(|(allr, _)| allr)
            .unwrap();
        let ingr = allergens
            .remove(unique)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        for (_, ing) in &mut allergens {
            ing.remove(&ingr);
        }
        concluded.insert(unique, ingr);
    }

    let mut canon: Vec<(&str, &str)> = concluded.into_iter().collect();
    canon.sort_by_key(|(allrg, _)| *allrg);

    canon
        .into_iter()
        .map(|(_, ingr)| ingr)
        .collect::<Vec<&str>>()
        .join(",")
}

fn parse(
    lines: &[String],
) -> (
    Vec<HashSet<&str>>,
    HashSet<&str>,
    HashMap<&str, HashSet<&str>>,
) {
    let mut recipes: Vec<HashSet<&str>> = vec![];
    let mut ingredients: HashSet<&str> = HashSet::new();
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in lines.iter().map(|l| l.trim()) {
        let mut parts = line.split('(');
        let ing: HashSet<&str> = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();

        if let Some(allrgs) = parts
            .next()
            .and_then(|s| s.strip_suffix(")"))
            .and_then(|s| s.strip_prefix("contains "))
        {
            for allrg in allrgs.split(", ") {
                if let Some(existing) = allergens.remove(allrg) {
                    allergens.insert(allrg, existing.intersection(&ing).copied().collect());
                } else {
                    allergens.insert(allrg, ing.clone());
                }
            }
        }

        ingredients.extend(ing.iter());
        recipes.push(ing);
    }

    (recipes, ingredients, allergens)
}

pub fn solve(lines: &[String]) -> Solution {
    let (recipes, ingredients, allergens) = parse(lines);
    (
        solve_a(&recipes, &ingredients, &allergens).to_string(),
        solve_b(allergens).to_string(),
    )
}
