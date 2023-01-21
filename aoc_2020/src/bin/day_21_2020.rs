use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    fn ingredients_for_allergen(&self, allergen: &str) -> Option<HashSet<String>> {
        if self.allergens.contains(&allergen.to_string()) {
            Some(self.ingredients.iter().cloned().collect())
        } else {
            None
        }
    }

    fn not_allergens(&self, allergens: &[&str]) -> usize {
        self.ingredients
            .iter()
            .filter(|ing| !allergens.contains(&ing.as_str()))
            .count()
    }
}

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" (contains ").collect();
        let ingredients: Vec<String> = parts[0].split_whitespace().map(|s| s.to_string()).collect();
        let allergens: Vec<String> = parts[1]
            .split(&[' ', ')', ','])
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            })
            .collect();
        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

#[derive(Debug)]
struct Menu {
    foods: Vec<Food>,
    allergens_set: HashSet<String>,
    allergens_map: HashMap<String, String>,
}

impl Menu {
    fn not_allergens(&self) -> usize {
        let allergens: Vec<&str> = self.allergens_map.values().map(|s| s.as_str()).collect();
        self.foods
            .iter()
            .map(|food| food.not_allergens(&allergens))
            .sum()
    }

    fn dangerous_ingredient_list(&self) -> String {
        let mut list: Vec<(String, String)> = self
            .allergens_map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        list.sort_by(|(a, _), (b, _)| a.cmp(b));
        list.into_iter().map(|(_, al)| al).join(",")
    }

    fn compute_allergens(&mut self) {
        let mut candidates_map: HashMap<String, HashSet<String>> = HashMap::new();
        for al in self.allergens_set.iter() {
            let candidates: HashSet<String> = self
                .foods
                .iter()
                .filter_map(|food| food.ingredients_for_allergen(al))
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap();
            if candidates.len() == 1 {
                self.allergens_map
                    .insert(al.clone(), candidates.iter().next().cloned().unwrap());
            } else {
                candidates_map.insert(al.clone(), candidates);
            }
        }

        //Build the final allergen map by removing other known allergens
        while !candidates_map.is_empty() {
            let mut to_remove: Vec<String> = Vec::new();
            for (al, set) in candidates_map.iter_mut() {
                self.allergens_map.values().for_each(|ing| {
                    set.remove(ing);
                });
                if set.len() == 1 {
                    self.allergens_map
                        .insert(al.clone(), set.iter().next().cloned().unwrap());
                    to_remove.push(al.clone());
                }
            }
            to_remove.into_iter().for_each(|al| {
                candidates_map.remove(&al);
            });
        }
    }
}

impl FromStr for Menu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let foods: Vec<Food> = s.lines().map(|l| l.parse().unwrap()).collect();
        let allergens_set: HashSet<String> = foods
            .iter()
            .flat_map(|food| food.allergens.clone())
            .collect();
        Ok(Self {
            foods,
            allergens_set,
            allergens_map: HashMap::new(),
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_21.txt").expect("Cannot open input file");

    let mut menu: Menu = s.parse().unwrap();
    menu.compute_allergens();
    println!(
        "Total number of not allergen ingredients: {}",
        menu.not_allergens()
    );
    println!(
        "The dangerous ingreident list is: {}",
        menu.dangerous_ingredient_list()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_1() {
        let mut menu: Menu = INPUT.parse().unwrap();
        menu.compute_allergens();
        assert_eq!(menu.not_allergens(), 5);
        assert_eq!(menu.dangerous_ingredient_list(), "mxmxvkd,sqjhc,fvjkl");
    }
}
