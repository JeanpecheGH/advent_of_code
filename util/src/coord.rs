use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos(pub usize, pub usize);
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PosI(pub isize, pub isize);
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos3(pub usize, pub usize, pub usize);
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos3I(pub isize, pub isize, pub isize);
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos4(pub usize, pub usize, pub usize, pub usize);

impl Pos {
    pub fn distance(&self, Pos(x, y): Pos) -> usize {
        self.0.abs_diff(x) + self.1.abs_diff(y)
    }

    pub fn neighbours_safe(&self, max_x: usize, max_y: usize) -> Vec<Pos> {
        let x: isize = self.0 as isize;
        let y: isize = self.1 as isize;
        let i = PosI(x, y);
        i.neighbours()
            .into_iter()
            .filter_map(|PosI(a, b)| {
                if a >= 0 && a <= max_x as isize && b >= 0 && b <= max_y as isize {
                    Some(Pos(a as usize, b as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn neighbours(&self) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![Pos(x - 1, y), Pos(x + 1, y), Pos(x, y - 1), Pos(x, y + 1)]
    }

    pub fn neighbours_diag(&self) -> Vec<Pos> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(i, j)| *i != 0 || *j != 0)
            .map(|(i, j)| {
                Pos(
                    (self.0 as isize + i) as usize,
                    (self.1 as isize + j) as usize,
                )
            })
            .collect()
    }
}

impl PosI {
    pub fn distance(&self, PosI(x, y): PosI) -> usize {
        self.0.abs_diff(x) + self.1.abs_diff(y)
    }

    pub fn neighbours(&self) -> Vec<PosI> {
        let &PosI(x, y) = self;
        vec![
            PosI(x - 1, y),
            PosI(x + 1, y),
            PosI(x, y - 1),
            PosI(x, y + 1),
        ]
    }

    pub fn neighbours_diag(&self) -> Vec<PosI> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(i, j)| *i != 0 || *j != 0)
            .map(|(i, j)| PosI(self.0 + i, self.1 + j))
            .collect()
    }
}
