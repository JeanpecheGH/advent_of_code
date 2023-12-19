use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, anychar, char};
use nom::combinator::{map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

#[derive(Copy, Clone, Debug)]
enum Category {
    Xcool,
    Music,
    Aero,
    Shiny,
}

impl Category {
    fn from_char(c: char) -> Result<Category, String> {
        match c {
            'x' => Ok(Category::Xcool),
            'm' => Ok(Category::Music),
            'a' => Ok(Category::Aero),
            's' => Ok(Category::Shiny),
            _ => Err(format!("Unknown Category [{c}]")),
        }
    }
}

#[derive(Clone, Debug)]
enum RuleResult {
    Accept,
    Reject,
    SendTo(String),
}

impl FromStr for RuleResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RuleResult::Accept),
            "R" => Ok(RuleResult::Reject),
            _ => Ok(RuleResult::SendTo(s.to_string())),
        }
    }
}

struct PartRule {
    cat: Option<Category>,
    sup: bool,
    value: usize,
    result: RuleResult,
}

impl PartRule {
    fn apply(&self, part: MachinePart) -> Option<RuleResult> {
        if let Some(c) = &self.cat {
            let b: bool = match (c, self.sup) {
                (Category::Xcool, true) => part.x > self.value,
                (Category::Xcool, false) => part.x < self.value,
                (Category::Music, true) => part.m > self.value,
                (Category::Music, false) => part.m < self.value,
                (Category::Aero, true) => part.a > self.value,
                (Category::Aero, false) => part.a < self.value,
                (Category::Shiny, true) => part.s > self.value,
                (Category::Shiny, false) => part.s < self.value,
            };
            if b {
                Some(self.result.clone())
            } else {
                None
            }
        } else {
            Some(self.result.clone())
        }
    }

    //Returns a (result, range) for the part passing the test  or if there is no condition
    //Also returns an optional range if the test failed
    fn apply_range(&self, part: PartRange) -> (RuleResult, PartRange, Option<PartRange>) {
        if let Some(c) = &self.cat {
            let (good, bad) = part.split(*c, self.sup, self.value);
            (self.result.clone(), good, Some(bad))
        } else {
            //No test, all the range applies the RuleResult
            (self.result.clone(), part, None)
        }
    }
}

impl FromStr for PartRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_start(s: &str) -> IResult<&str, (Category, bool, usize)> {
            let (s, cat) = map_res(anychar, Category::from_char)(s)?;
            let (s, sup) = map(anychar, |c| c == '>')(s)?;
            let (s, value) = terminated(parse_usize, char(':'))(s)?;
            Ok((s, (cat, sup, value)))
        }

        fn parse_part_rule(s: &str) -> IResult<&str, PartRule> {
            //println!("Parsing part rule: {s}");
            let (s, triplet) = opt(parse_start)(s)?;
            //println!("Parsing part rule, triplet: {triplet:?}");
            let (s, result) = map_res(alpha1, RuleResult::from_str)(s)?;

            if let Some((cat, sup, value)) = triplet {
                Ok((
                    s,
                    PartRule {
                        cat: Some(cat),
                        sup,
                        value,
                        result,
                    },
                ))
            } else {
                Ok((
                    s,
                    PartRule {
                        cat: None,
                        sup: true,
                        value: 0,
                        result,
                    },
                ))
            }
        }
        Ok(parse_part_rule(s).unwrap().1)
    }
}

struct Workflow {
    id: String,
    tests: Vec<PartRule>,
}

impl Workflow {
    fn apply_rules(&self, part: MachinePart) -> RuleResult {
        for rule in self.tests.iter() {
            match rule.apply(part) {
                None => (),
                Some(r) => return r,
            }
        }
        RuleResult::Reject
    }

    fn apply_rules_range(&self, range: PartRange) -> (Vec<PartRange>, Vec<(String, PartRange)>) {
        let mut done: Vec<PartRange> = Vec::new();
        let mut todo: Vec<(String, PartRange)> = Vec::new();

        let mut remaining: Option<PartRange> = Some(range);
        for rule in self.tests.iter() {
            if let Some(remain) = remaining {
                let (r, d, opt): (RuleResult, PartRange, Option<PartRange>) =
                    rule.apply_range(remain);

                match r {
                    RuleResult::Accept => done.push(d),
                    RuleResult::Reject => (), //Nothing, already rejected
                    RuleResult::SendTo(id) => todo.push((id, d)),
                }
                remaining = opt;
            }
        }
        (done, todo)
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_workflow(s: &str) -> IResult<&str, Workflow> {
            //println!("Parsing wf: {s}");
            let (s, id) = alpha1(s)?;
            //println!("id: {id}");
            let (s, tests) = delimited(
                char('{'),
                separated_list1(
                    char(','),
                    map_res(take_till(|c| c == ',' || c == '}'), PartRule::from_str),
                ),
                char('}'),
            )(s)?;
            Ok((
                s,
                Workflow {
                    id: id.to_string(),
                    tests,
                },
            ))
        }
        Ok(parse_workflow(s).unwrap().1)
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn valid(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    //Return 2 ranges
    //The left one applies the condition, the right one does not
    fn split(&self, c: Category, sup: bool, value: usize) -> (PartRange, PartRange) {
        let mut good = self.clone();
        let mut bad = self.clone();
        match (c, sup) {
            (Category::Xcool, true) => {
                good.x.start = value + 1;
                bad.x.end = value + 1;
            }
            (Category::Xcool, false) => {
                good.x.end = value;
                bad.x.start = value;
            }
            (Category::Music, true) => {
                good.m.start = value + 1;
                bad.m.end = value + 1;
            }
            (Category::Music, false) => {
                good.m.end = value;
                bad.m.start = value;
            }
            (Category::Aero, true) => {
                good.a.start = value + 1;
                bad.a.end = value + 1;
            }
            (Category::Aero, false) => {
                good.a.end = value;
                bad.a.start = value;
            }
            (Category::Shiny, true) => {
                good.s.start = value + 1;
                bad.s.end = value + 1;
            }
            (Category::Shiny, false) => {
                good.s.end = value;
                bad.s.start = value;
            }
        }
        (good, bad)
    }
}

#[derive(Copy, Clone, Debug)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachinePart {
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for MachinePart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_part(s: &str) -> IResult<&str, MachinePart> {
            let (rs, x) = preceded(tag("{x="), parse_usize)(s)?;
            let (rs, m) = preceded(tag(",m="), parse_usize)(rs)?;
            let (rs, a) = preceded(tag(",a="), parse_usize)(rs)?;
            let (rs, s) = preceded(tag(",s="), parse_usize)(rs)?;

            Ok((rs, MachinePart { x, m, a, s }))
        }
        Ok(parse_part(s).unwrap().1)
    }
}

struct XmasSorter {
    workflows: HashMap<String, Workflow>,
    parts: Vec<MachinePart>,
}

impl XmasSorter {
    fn valid_parts(&self) -> usize {
        let start_range: PartRange = PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };

        let mut valid_ranges: Vec<PartRange> = Vec::new();
        let mut todo_ranges: Vec<(String, PartRange)> = vec![("in".to_string(), start_range)];

        while let Some((id, range)) = todo_ranges.pop() {
            let flow: &Workflow = self.workflows.get(&id).unwrap();
            let (done, todo) = flow.apply_rules_range(range);
            valid_ranges.extend(done);
            todo_ranges.extend(todo);
        }

        valid_ranges.into_iter().map(|r| r.valid()).sum()
    }
    fn ratings(&self) -> usize {
        let mut sum: usize = 0;

        for p in self.parts.iter() {
            let mut flow: &Workflow = self.workflows.get("in").unwrap();
            while let RuleResult::SendTo(new_id) = flow.apply_rules(*p) {
                flow = self.workflows.get(&new_id).unwrap();
            }

            match flow.apply_rules(*p) {
                RuleResult::Accept => sum += p.rating(),
                RuleResult::Reject => (),
                RuleResult::SendTo(_) => (),
            }
        }

        sum
    }
}

impl FromStr for XmasSorter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = split_blocks(s);
        let workflows: HashMap<String, Workflow> = blocks[0]
            .lines()
            .map(|l| {
                //println!("Parsing: {l}");
                let workflow: Workflow = l.parse().unwrap();
                (workflow.id.clone(), workflow)
            })
            .collect();

        let parts: Vec<MachinePart> = blocks[1].lines().map(|l| l.parse().unwrap()).collect();
        Ok(XmasSorter { workflows, parts })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_19.txt").expect("Cannot open input file");
    let sorter: XmasSorter = s.parse().unwrap();
    println!(
        "Part1: The sum of the ratings of the accepted part is {}",
        sorter.ratings()
    );
    println!(
        "Part2: {} distinct parts could be accepted by the sorting process",
        sorter.valid_parts()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn part_1() {
        let sorter: XmasSorter = EXAMPLE_1.parse().unwrap();
        assert_eq!(sorter.ratings(), 19114);
    }
    #[test]
    fn part_2_test_1() {
        let sorter: XmasSorter = EXAMPLE_1.parse().unwrap();
        assert_eq!(sorter.valid_parts(), 167409079868000);
    }
}
