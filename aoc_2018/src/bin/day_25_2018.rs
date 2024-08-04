use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::coord::Pos4I;

const DENSITY: usize = 3;

#[derive(Debug, Clone)]
struct StarrySky {
    stars: Vec<Pos4I>,
}

impl StarrySky {
    fn constellations(&self) -> usize {
        let mut constellations: Vec<Vec<Pos4I>> = Vec::new();

        for &star in self.stars.iter() {
            //Partition in constellations that are in range of our star and those that are not
            let (in_range, mut out_of_range): (Vec<Vec<Pos4I>>, Vec<Vec<Pos4I>>) = constellations
                .into_iter()
                .partition(|c| c.iter().any(|s| s.distance(star) <= DENSITY));
            //Merge the constellations in range and add our new star to it
            let mut merge: Vec<Pos4I> = in_range.into_iter().flatten().collect();
            merge.push(star);
            out_of_range.push(merge);
            constellations = out_of_range;
        }

        constellations.len()
    }
}

impl FromStr for StarrySky {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_star(s: &str) -> IResult<&str, Pos4I> {
            let (s, v) = separated_list1(char(','), parse_isize)(s)?;

            Ok((s, Pos4I(v[0], v[1], v[2], v[3])))
        }

        let stars: Vec<Pos4I> = s.lines().map(|l| parse_star(l).unwrap().1).collect();

        Ok(StarrySky { stars })
    }
}

fn main() {
    let now = std::time::Instant::now();

    let s = util::file_as_string("aoc_2018/input/day_25.txt").expect("Cannot open input file");
    let sky: StarrySky = s.parse().unwrap();

    println!("Part1: The {}", sky.constellations());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
";

    const EXAMPLE_2: &str = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
";
    const EXAMPLE_3: &str = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
";
    const EXAMPLE_4: &str = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
";

    #[test]
    fn test_1() {
        let sky: StarrySky = EXAMPLE_1.parse().unwrap();
        assert_eq!(2, sky.constellations());
    }

    #[test]
    fn test_2() {
        let sky: StarrySky = EXAMPLE_2.parse().unwrap();
        assert_eq!(4, sky.constellations());
    }

    #[test]
    fn test_3() {
        let sky: StarrySky = EXAMPLE_3.parse().unwrap();
        assert_eq!(3, sky.constellations());
    }

    #[test]
    fn test_4() {
        let sky: StarrySky = EXAMPLE_4.parse().unwrap();
        assert_eq!(8, sky.constellations());
    }
}
