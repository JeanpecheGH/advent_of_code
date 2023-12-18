#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'U' |'N' => Ok(Dir::North),
            'L' |'W' => Ok(Dir::West),
            'D' |'S' => Ok(Dir::South),
            'R' |'E' => Ok(Dir::East),
            _ => Err(format!("Invalid Direction [{c}]"))
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
    pub fn turn_left(&self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    pub fn half_turn(&self) -> Self {
        self.turn_left().turn_left()
    }
}
