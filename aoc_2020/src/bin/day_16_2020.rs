use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct Field {
    name: String,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>),
}

impl Field {
    fn contains(&self, n: &usize) -> bool {
        self.ranges.0.contains(n) || self.ranges.1.contains(n)
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(':').collect();
        let name: String = words[0].to_string();
        let numbers: Vec<&str> = words[1].split(&[' ', '-']).collect();
        let n1: usize = numbers[1].parse().unwrap();
        let n2: usize = numbers[2].parse().unwrap();
        let n3: usize = numbers[4].parse().unwrap();
        let n4: usize = numbers[5].parse().unwrap();
        let ranges: (RangeInclusive<usize>, RangeInclusive<usize>) = (n1..=n2, n3..=n4);
        Ok(Field { name, ranges })
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<usize>,
}

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            values: s.split(',').map(|w| w.parse().unwrap()).collect(),
        })
    }
}

#[derive(Debug)]
struct TicketSystem {
    fields: Vec<Field>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

impl TicketSystem {
    fn ticket_score(&self) -> usize {
        //Find all valid columns for each Field
        let field_set: HashSet<usize> = (0..self.fields.len()).collect();
        let mut candidate_columns: Vec<(String, HashSet<usize>)> = Vec::new();
        self.fields.iter().for_each(|field| {
            let candidate_set: HashSet<usize> = field_set
                .iter()
                .filter(|idx| {
                    self.tickets
                        .iter()
                        .chain(vec![&self.my_ticket])
                        .map(|t| t.values[**idx])
                        .all(|v| field.contains(&v))
                })
                .cloned()
                .collect();
            candidate_columns.push((field.name.clone(), candidate_set));
        });

        //Sort the elements by the number of valid column
        //Each field has 1 more valid column compared to the previous one, this is the one we get
        let mut found_columns: HashSet<usize> = HashSet::new();
        let mut sorted_names: Vec<String> = vec!["".to_string(); self.fields.len()];
        candidate_columns.sort_by(|(_, set_a), (_, set_b)| set_a.len().cmp(&set_b.len()));
        candidate_columns.into_iter().for_each(|(name, columns)| {
            let idx: usize = columns
                .into_iter()
                .find(|col| !found_columns.contains(col))
                .unwrap();
            sorted_names[idx] = name;
            found_columns.insert(idx);
        });

        //Keep the "departure" fields in your ticket and multiply
        self.my_ticket
            .values
            .iter()
            .enumerate()
            .filter(|(i, _)| sorted_names[*i].starts_with("departure"))
            .map(|(_, v)| *v)
            .product()
    }

    fn error_rate(&self) -> usize {
        self.tickets
            .iter()
            .map(|ticket| {
                ticket
                    .values
                    .iter()
                    .filter(|v| self.fields.iter().all(|field| !field.contains(v)))
                    .cloned()
                    .sum::<usize>()
            })
            .sum()
    }

    fn filter_errors(&mut self) {
        self.tickets.retain(|ticket| {
            ticket
                .values
                .iter()
                .all(|v| self.fields.iter().any(|field| field.contains(v)))
        });
    }
}

impl FromStr for TicketSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let groups: Vec<&[&str]> = lines.split(|l| l.is_empty()).collect();
        let fields: Vec<Field> = groups[0].iter().map(|l| l.parse().unwrap()).collect();
        let my_ticket: Ticket = groups[1].last().map(|l| l.parse().unwrap()).unwrap();
        let tickets: Vec<&str> = groups[2][1..].to_vec();
        let tickets: Vec<Ticket> = tickets.into_iter().filter_map(|l| l.parse().ok()).collect();
        Ok(TicketSystem {
            fields,
            my_ticket,
            tickets,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_16.txt").expect("Cannot open input file");
    let mut system: TicketSystem = s.parse().unwrap();
    println!(
        "Part1: The ticket scanning error rate is {}",
        system.error_rate()
    );
    system.filter_errors();
    println!(
        "Part2: The product of the departure fields on your ticket is {}",
        system.ticket_score()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn part_1() {
        let system: TicketSystem = INPUT.parse().unwrap();
        assert_eq!(system.error_rate(), 71);
    }
}
