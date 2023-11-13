use util::coord::PosI;
use util::intcode::IntCode;

struct TractorBeam {
    intcode: IntCode,
}

impl TractorBeam {
    fn field_at(&mut self, x: isize, y: isize) -> isize {
        self.intcode.compute(&mut vec![x, y]);
        let out: isize = self.intcode.output.pop().unwrap();
        self.intcode.reset();
        out
    }

    fn points_in_area(&mut self, x_max: isize, y_max: isize) -> isize {
        (0..y_max)
            .map(|y| (0..x_max).map(|x| self.field_at(x, y)).sum::<isize>())
            .sum()
    }

    fn find_ship(&mut self, ship_size: isize, start: isize) -> PosI {
        let true_size = ship_size - 1;
        let mut y = start;
        let mut x = y / 2;
        loop {
            if self.field_at(x, y) == 1 {
                if self.field_at(x + true_size, y - true_size) == 1 {
                    return PosI(x, y - true_size);
                }
                y += 1;
            } else {
                x += 1;
            }
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_19.txt").expect("Cannot open input file");
    let intcode: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut beam: TractorBeam = TractorBeam { intcode };

    println!(
        "Part 1: {} points are affected by the tractor beam",
        beam.points_in_area(50, 50)
    );

    //Starting arbitrary at 200
    let ship = beam.find_ship(100, 200);
    println!(
        "Part 2: Ship position is {:?}, score = {}",
        ship,
        ship.0 * 10000 + ship.1
    );

    println!("Computing time: {:?}", now.elapsed());
}
