struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn mult(&self, fact: i64) -> Self {
        Ingredient {
            capacity: self.capacity * fact,
            durability: self.durability * fact,
            flavor: self.flavor * fact,
            texture: self.texture * fact,
            calories: self.calories * fact,
        }
    }
    fn add(&self, other: Ingredient) -> Self {
        Ingredient {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
    fn score(&self, with_calories: bool) -> i64 {
        if with_calories {
            return self.score_with_calories();
        }
        match (self.capacity, self.durability, self.flavor, self.texture) {
            (i64::MIN..=0, _, _, _) => 0,
            (_, i64::MIN..=0, _, _) => 0,
            (_, _, i64::MIN..=0, _) => 0,
            (_, _, _, i64::MIN..=0) => 0,
            (a, b, c, d) => a * b * c * d,
        }
    }
    fn score_with_calories(&self) -> i64 {
        match (
            self.calories,
            self.capacity,
            self.durability,
            self.flavor,
            self.texture,
        ) {
            (_, i64::MIN..=0, _, _, _) => 0,
            (_, _, i64::MIN..=0, _, _) => 0,
            (_, _, _, i64::MIN..=0, _) => 0,
            (_, _, _, _, i64::MIN..=0) => 0,
            (500, a, b, c, d) => a * b * c * d,
            _ => 0,
        }
    }
}

fn main() {
    let s = util::file_as_string("aoc_2015/input/day_15.txt").expect("Cannot open input file");

    let ingredients: Vec<Ingredient> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split(' ').collect();

            let capacity: i64 = words[2].strip_suffix(',').unwrap().parse::<i64>().unwrap();
            let durability: i64 = words[4].strip_suffix(',').unwrap().parse::<i64>().unwrap();
            let flavor: i64 = words[6].strip_suffix(',').unwrap().parse::<i64>().unwrap();
            let texture: i64 = words[8].strip_suffix(',').unwrap().parse::<i64>().unwrap();
            let calories: i64 = words[10].parse().unwrap();
            Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
        .collect();

    let res = cookie_scores(ingredients.as_slice(), 100, false);
    println!("Part1: Max cookie score is {res}");
    let res_2 = cookie_scores(ingredients.as_slice(), 100, true);
    println!("Part2: Max cookie score is {res_2} with a limit of 500 calories");
}

fn cookie_scores(ingredients: &[Ingredient], total_spoons: i64, with_calories: bool) -> i64 {
    let mut max_score = 0;
    for i in 0..=total_spoons {
        for j in 0..=(total_spoons - i) {
            for k in 0..=(total_spoons - i - j) {
                let l = total_spoons - i - j - k;
                let score = score(ingredients, vec![i, j, k, l], with_calories);
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }
    max_score
}

fn score(ingredients: &[Ingredient], spoons: Vec<i64>, with_calories: bool) -> i64 {
    ingredients
        .iter()
        .zip(spoons)
        .map(|(ing, spoon)| ing.mult(spoon))
        .reduce(|a, b| a.add(b))
        .map(|ing| ing.score(with_calories))
        .unwrap()
}
