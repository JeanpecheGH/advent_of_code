use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::coord::Pos3I;
use util::split_blocks;

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Pos3I>,
}

impl Scanner {
    fn all_rotations(&self) -> Vec<Vec<Pos3I>> {
        fn rot_x(beacons: &[Pos3I]) -> Vec<Pos3I> {
            beacons
                .iter()
                .map(|&Pos3I(x, y, z)| Pos3I(x, -z, y))
                .collect()
        }

        fn rot_y(beacons: &[Pos3I]) -> Vec<Pos3I> {
            beacons
                .iter()
                .map(|&Pos3I(x, y, z)| Pos3I(z, y, -x))
                .collect()
        }

        fn rot_z(beacons: &[Pos3I]) -> Vec<Pos3I> {
            beacons
                .iter()
                .map(|&Pos3I(x, y, z)| Pos3I(y, -x, z))
                .collect()
        }

        let i = self.beacons.clone();
        let x = rot_x(&i);
        let y = rot_y(&i);
        let z = rot_z(&i);
        let xx = rot_x(&x);
        let xy = rot_y(&x);
        let xz = rot_z(&x);
        let yx = rot_x(&y);
        let yy = rot_y(&y);
        let zy = rot_y(&z);
        let zz = rot_z(&z);
        let xxx = rot_x(&xx);
        let xxy = rot_y(&xx);
        let xxz = rot_z(&xx);
        let xyx = rot_x(&xy);
        let xyy = rot_y(&xy);
        let xzz = rot_z(&xz);
        let yxx = rot_x(&yx);
        let yyy = rot_y(&yy);
        let zzz = rot_z(&zz);
        let xxxy = rot_y(&xxx);
        let xxyx = rot_x(&xxy);
        let xyxx = rot_x(&xyx);
        let xyyy = rot_y(&xyy);
        vec![
            i, x, y, z, xx, xy, xz, yx, yy, zy, zz, xxx, xxy, xxz, xyx, xyy, xzz, yxx, yyy, zzz,
            xxxy, xxyx, xyxx, xyyy,
        ]
    }
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos3i(s: &str) -> IResult<&str, Pos3I> {
            let (s, v) = separated_list1(char(','), parse_isize).parse(s)?;

            Ok((s, Pos3I(v[0], v[1], v[2])))
        }
        let beacons: Vec<Pos3I> = s
            .lines()
            .skip(1)
            .map(|l| parse_pos3i(l).unwrap().1)
            .collect();
        Ok(Scanner { beacons })
    }
}

#[derive(Debug, Clone)]
struct ScannerFleet {
    scanners: Vec<Scanner>,
}

impl ScannerFleet {
    fn solve(&self) -> (usize, usize) {
        fn compare_beacons(
            source_beacons: &[Pos3I],
            target_beacons: &[Pos3I],
        ) -> Option<(Pos3I, Vec<Pos3I>)> {
            //Compute all the difference between any 2 beacon
            //Search for a group of 12 or more differences that are equals. This means it is a good orientation
            if let Some((target_pos, _)) = source_beacons
                .iter()
                .flat_map(|src| target_beacons.iter().map(|tgt| src.sub(*tgt)))
                .counts()
                .into_iter()
                .find(|&(_, c)| c >= 12)
            {
                Some((
                    target_pos,
                    target_beacons.iter().map(|b| b.add(target_pos)).collect(),
                ))
            } else {
                None
            }
        }

        let mut found_scanner: Vec<bool> = vec![false; self.scanners.len()];
        let mut final_positions: Vec<(Pos3I, Vec<Pos3I>)> =
            vec![(Pos3I(0, 0, 0), Vec::new()); self.scanners.len()];
        final_positions[0] = (Pos3I(0, 0, 0), self.scanners[0].beacons.clone());
        let mut to_compute: Vec<usize> = vec![0];

        let mut rotations_map: FxHashMap<usize, Vec<Vec<Pos3I>>> = FxHashMap::default();

        //End when all scanners have been located
        while found_scanner.iter().any(|&found| !found) {
            let source: usize = to_compute.pop().unwrap();
            let source_beacons: &[Pos3I] = &final_positions[source].1;

            let mut found_pairs: Vec<(usize, (Pos3I, Vec<Pos3I>))> = Vec::new();

            for (i, found) in found_scanner.iter_mut().enumerate() {
                //Only compare to scanner that are not yet located
                if !*found {
                    let rotations = rotations_map
                        .entry(i)
                        .or_insert(self.scanners[i].all_rotations());
                    //Compare the source to all the rotations of the target
                    if let Some(pair) = rotations
                        .iter()
                        .find_map(|target_group| compare_beacons(source_beacons, target_group))
                    {
                        to_compute.push(i);
                        *found = true;
                        found_pairs.push((i, pair));
                    }
                }
            }

            for (i, pair) in found_pairs {
                final_positions[i] = pair;
            }
        }

        let beacons_set: FxHashSet<&Pos3I> = final_positions
            .iter()
            .flat_map(|(_, beacons)| beacons)
            .collect();

        let max_distance: usize = final_positions
            .iter()
            .flat_map(|(source, _)| {
                final_positions
                    .iter()
                    .map(|(target, _)| source.distance(*target))
            })
            .max()
            .unwrap();

        (beacons_set.len(), max_distance)
    }
}

impl FromStr for ScannerFleet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanners: Vec<Scanner> = split_blocks(s)
            .into_iter()
            .map(|block| block.parse().unwrap())
            .collect();
        Ok(ScannerFleet { scanners })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_19.txt").expect("Cannot open input file");
    let fleet: ScannerFleet = s.parse().unwrap();
    let (nb_beacons, max_scanner_distance) = fleet.solve();
    println!("Part1: There are {nb_beacons} distinct beacons");
    println!("Part2: The largest distance between two scanners is {max_scanner_distance}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    #[test]
    fn test() {
        let fleet: ScannerFleet = EXAMPLE_1.parse().unwrap();
        assert_eq!((79, 3621), fleet.solve());
    }
}
