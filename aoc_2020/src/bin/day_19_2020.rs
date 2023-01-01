use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Expression {
    Value(String),
    Ref(usize),
    RefPair(usize, usize),
    RefTriple(usize, usize, usize),
}

#[derive(Debug, Clone)]
struct Rule {
    id: usize,
    has_ref: bool,
    exps: Vec<Expression>,
}

impl Rule {
    fn match_msg(&self, msg: &str) -> bool {
        self.exps
            .iter()
            .any(|exp| matches!(exp, Expression::Value(v) if v == msg))
    }

    fn get_refs(&self) -> HashSet<usize> {
        let set: HashSet<usize> = self
            .exps
            .iter()
            .flat_map(|exp| match exp {
                Expression::Value(_) => Vec::new(),
                Expression::Ref(a) => vec![*a],
                Expression::RefPair(a, b) => vec![*a, *b],
                Expression::RefTriple(a, b, c) => vec![*a, *b, *c],
            })
            .collect();

        set
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&[':', ' ', '\"']).collect();
        let id: usize = words[0].parse().unwrap();
        let mut has_ref: bool = true;
        let mut exps: Vec<Expression> = Vec::new();
        match words.len() {
            5 if words[2].is_empty() => {
                //Direct expression
                has_ref = false;
                exps.push(Expression::Value(words[3].to_string()));
            }
            5 if words[3] == "|" => {
                //2 Single refs
                let first: usize = words[2].parse().unwrap();
                exps.push(Expression::Ref(first));
                let second: usize = words[4].parse().unwrap();
                exps.push(Expression::Ref(second));
            }
            5 => {
                //Triple ref
                let first: usize = words[2].parse().unwrap();
                let second: usize = words[3].parse().unwrap();
                let third: usize = words[4].parse().unwrap();
                exps.push(Expression::RefTriple(first, second, third));
            }
            3 => {
                //Single ref
                let first: usize = words[2].parse().unwrap();
                exps.push(Expression::Ref(first));
            }
            4 => {
                //Pair of ref
                let first: usize = words[2].parse().unwrap();
                let second: usize = words[3].parse().unwrap();
                exps.push(Expression::RefPair(first, second));
            }
            _ => {
                //2 Pairs of ref
                let first: usize = words[2].parse().unwrap();
                let second: usize = words[3].parse().unwrap();
                exps.push(Expression::RefPair(first, second));
                let third: usize = words[5].parse().unwrap();
                let fourth: usize = words[6].parse().unwrap();
                exps.push(Expression::RefPair(third, fourth));
            }
        }
        Ok(Rule { id, has_ref, exps })
    }
}

#[derive(Debug)]
struct MessagesAndRules {
    rules: Vec<Rule>,
    messages: Vec<String>,
}

impl MessagesAndRules {
    fn nb_match_part_1(&self) -> usize {
        let r: &Rule = &self.rules[0];
        self.messages.iter().filter(|msg| r.match_msg(msg)).count()
    }

    fn nb_match_part_2(&self, chunk_len: usize) -> usize {
        let r_42: &Rule = &self.rules[42];
        let r_31: &Rule = &self.rules[31];
        self.messages
            .iter()
            .filter(|msg| {
                let mut ok: bool = true;
                let mut first_rule: bool = true;
                let mut nb_42: usize = 0;
                let mut nb_31: usize = 0;
                for i in (0..msg.len()).step_by(chunk_len) {
                    let chunk: &str = &msg[i..i + chunk_len];
                    if first_rule {
                        if r_42.match_msg(chunk) {
                            nb_42 += 1;
                        } else if r_31.match_msg(chunk) && nb_42 >= 2 {
                            nb_31 += 1;
                            first_rule = false;
                        } else {
                            ok = false;
                            break;
                        }
                    } else if r_31.match_msg(chunk) {
                        nb_31 += 1;
                    } else {
                        ok = false;
                        break;
                    }
                }
                if nb_31 == 0 || nb_42 <= nb_31 {
                    ok = false
                }
                ok
            })
            .count()
    }

    fn compute_rules(&mut self, part_two: bool) {
        self.rules.sort_by(|a, b| a.id.cmp(&b.id));

        while self.rules.iter().any(|r| r.has_ref) {
            for i in 0..self.rules.len() {
                let mut rule: Rule = self.rules[i].clone();
                if rule.has_ref
                    && rule
                        .get_refs()
                        .iter()
                        .all(|ref_id| !self.rules[*ref_id].has_ref)
                {
                    let new_exps: Vec<Expression> = rule
                        .exps
                        .into_iter()
                        .flat_map(|exp| match exp {
                            Expression::Value(_) => vec![exp],
                            Expression::Ref(a) => self.format_single(a, i, part_two),
                            Expression::RefPair(a, b) => self.format_pair(a, b, i, part_two),
                            Expression::RefTriple(a, b, c) => self.format_triple(a, b, c),
                        })
                        .collect();
                    rule.has_ref = false;
                    rule.exps = new_exps;
                    self.rules[i] = rule;
                }
            }
        }
    }

    fn format_single(&self, a: usize, i: usize, part_two: bool) -> Vec<Expression> {
        if part_two && i == 8 {
            //Make the rule "8: 42" into "8: 42 8"
            Vec::new()
        } else {
            self.rules[a].exps.clone()
        }
    }

    fn format_pair(&self, a: usize, b: usize, i: usize, part_two: bool) -> Vec<Expression> {
        if part_two && i == 11 {
            //Make the rule "11: 42 31" into "11: 42 31 | 42 11 31"
            Vec::new()
        } else {
            self.rules[a]
                .exps
                .iter()
                .cartesian_product(self.rules[b].exps.iter())
                .filter_map(|(exp_a, exp_b)| match (exp_a, exp_b) {
                    (Expression::Value(val_a), Expression::Value(val_b)) => {
                        Some(Expression::Value(format!("{val_a}{val_b}")))
                    }
                    _ => None,
                })
                .collect()
        }
    }

    fn format_triple(&self, a: usize, b: usize, c: usize) -> Vec<Expression> {
        self.rules[a]
            .exps
            .iter()
            .cartesian_product(
                self.rules[b]
                    .exps
                    .iter()
                    .cartesian_product(self.rules[c].exps.iter()),
            )
            .filter_map(|(exp_a, (exp_b, exp_c))| match (exp_a, exp_b, exp_c) {
                (Expression::Value(val_a), Expression::Value(val_b), Expression::Value(val_c)) => {
                    Some(Expression::Value(format!("{val_a}{val_b}{val_c}")))
                }
                _ => None,
            })
            .collect()
    }
}

impl FromStr for MessagesAndRules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let groups: Vec<&[&str]> = lines.split(|l| l.is_empty()).collect();
        let rules: Vec<Rule> = groups[0].iter().map(|l| l.parse().unwrap()).collect();
        let messages: Vec<String> = groups[1].iter().map(|l| l.to_string()).collect();
        Ok(MessagesAndRules { rules, messages })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_19.txt").expect("Cannot open input file");
    let mut msg_rules: MessagesAndRules = s.parse().unwrap();
    msg_rules.compute_rules(false);
    println!(
        "Part1: There are {} messages matching rule 0",
        msg_rules.nb_match_part_1()
    );
    println!(
        "Part2: There are {} messages matching rule 0 with updated rules 8 and 11",
        msg_rules.nb_match_part_2(8)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    const INPUT_2: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
29: 1
30: 1
32: 1
33: 1
34: 1
35: 1
36: 1
37: 1
38: 1
39: 1
40: 1
41: 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_1() {
        let mut msg_rules: MessagesAndRules = INPUT.parse().unwrap();
        msg_rules.compute_rules(false);
        assert_eq!(msg_rules.nb_match_part_1(), 2);
    }

    #[test]
    fn test_2() {
        let mut msg_rules: MessagesAndRules = INPUT_2.parse().unwrap();
        msg_rules.compute_rules(false);
        assert_eq!(msg_rules.nb_match_part_1(), 3);
    }
    #[test]
    fn test_2_part_2() {
        let mut msg_rules: MessagesAndRules = INPUT_2.parse().unwrap();
        msg_rules.compute_rules(true);
        assert_eq!(msg_rules.nb_match_part_2(5), 12);
    }
}
