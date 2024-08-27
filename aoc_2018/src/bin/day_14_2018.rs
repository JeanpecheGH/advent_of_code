struct RecipeBook {
    recipes: Vec<usize>,
    first_elf: usize,
    second_elf: usize,
}

impl RecipeBook {
    fn new() -> Self {
        let recipes: Vec<usize> = vec![3, 7];
        RecipeBook {
            recipes,
            first_elf: 0,
            second_elf: 1,
        }
    }

    fn next(&mut self) {
        let first: usize = self.recipes[self.first_elf];
        let second: usize = self.recipes[self.second_elf];
        let sum: usize = first + second;
        if sum >= 10 {
            self.recipes.push(1);
        }
        self.recipes.push(sum % 10);

        let len = self.recipes.len();
        self.first_elf = (self.first_elf + first + 1) % len;
        self.second_elf = (self.second_elf + second + 1) % len;
    }

    fn ten_scores_after(&mut self, n: usize) -> usize {
        while self.recipes.len() < n + 10 {
            self.next();
        }

        self.recipes[n..n + 10]
            .iter()
            .fold(0, |acc, n| acc * 10 + n)
    }

    fn recipes_before(&mut self, pattern_str: &str) -> usize {
        let pattern: Vec<usize> = pattern_str
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let pat_len: usize = pattern.len();

        loop {
            self.next();
            let len: usize = self.recipes.len();
            if len > pat_len && self.recipes[len - pat_len..] == pattern {
                return len - pat_len;
            }
            if len > pat_len + 1 && self.recipes[len - pat_len - 1..len - 1] == pattern {
                return len - pat_len - 1;
            }
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let mut book = RecipeBook::new();
    println!(
        "Part1: The scores of the ten recipes after the 554401 first recipes is {}",
        book.ten_scores_after(554401)
    );
    println!(
        "Part2: {} recipes are on the left when we first see the sequence 554401",
        book.recipes_before("554401")
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let mut book = RecipeBook::new();
        assert_eq!(format!("{:0>10}", book.ten_scores_after(9)), "5158916779");
    }

    #[test]
    fn part_1_test_2() {
        let mut book = RecipeBook::new();
        assert_eq!(format!("{:0>10}", book.ten_scores_after(5)), "0124515891");
    }

    #[test]
    fn part_1_test_3() {
        let mut book = RecipeBook::new();
        assert_eq!(format!("{:0>10}", book.ten_scores_after(18)), "9251071085");
    }

    #[test]
    fn part_1_test_4() {
        let mut book = RecipeBook::new();
        assert_eq!(
            format!("{:0>10}", book.ten_scores_after(2018)),
            "5941429882"
        );
    }

    #[test]
    fn part_2_test_1() {
        let mut book = RecipeBook::new();
        assert_eq!(book.recipes_before("51589"), 9);
    }

    #[test]
    fn part_2_test_2() {
        let mut book = RecipeBook::new();
        assert_eq!(book.recipes_before("01245"), 5);
    }

    #[test]
    fn part_2_test_3() {
        let mut book = RecipeBook::new();
        assert_eq!(book.recipes_before("92510"), 18);
    }

    #[test]
    fn part_2_test_4() {
        let mut book = RecipeBook::new();
        assert_eq!(book.recipes_before("59414"), 2018);
    }
}
