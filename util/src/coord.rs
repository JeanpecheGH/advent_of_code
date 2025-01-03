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
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos4I(pub isize, pub isize, pub isize, pub isize);

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
                if a >= 0 && a < max_x as isize && b >= 0 && b < max_y as isize {
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

    pub fn neighbours_diag_safe(&self, max_x: usize, max_y: usize) -> Vec<Pos> {
        let x: isize = self.0 as isize;
        let y: isize = self.1 as isize;
        let i = PosI(x, y);
        i.neighbours_diag()
            .into_iter()
            .filter_map(|PosI(a, b)| {
                if a >= 0 && a < max_x as isize && b >= 0 && b < max_y as isize {
                    Some(Pos(a as usize, b as usize))
                } else {
                    None
                }
            })
            .collect()
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

    pub fn neighbours_diag_limit(&self, max_x: isize, max_y: isize) -> Vec<PosI> {
        self.neighbours_diag()
            .into_iter()
            .filter(|&PosI(x, y)| x >= 0 && x < max_x && y >= 0 && y < max_y)
            .collect()
    }

    pub fn sub(&self, PosI(x, y): PosI) -> PosI {
        PosI(self.0 - x, self.1 - y)
    }

    pub fn add(&self, PosI(x, y): PosI) -> PosI {
        PosI(self.0 + x, self.1 + y)
    }
}

impl Pos3I {
    pub fn distance(&self, Pos3I(x, y, z): Pos3I) -> usize {
        self.0.abs_diff(x) + self.1.abs_diff(y) + self.2.abs_diff(z)
    }

    pub fn sub(&self, Pos3I(x, y, z): Pos3I) -> Pos3I {
        Pos3I(self.0 - x, self.1 - y, self.2 - z)
    }

    pub fn add(&self, Pos3I(x, y, z): Pos3I) -> Pos3I {
        Pos3I(self.0 + x, self.1 + y, self.2 + z)
    }
}

impl Pos4I {
    pub fn distance(&self, Pos4I(w, x, y, z): Pos4I) -> usize {
        self.0.abs_diff(w) + self.1.abs_diff(x) + self.2.abs_diff(y) + self.3.abs_diff(z)
    }
}
