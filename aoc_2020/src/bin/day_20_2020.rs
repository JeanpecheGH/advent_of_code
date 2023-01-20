use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;

const TILE_SIZE: usize = 10;
const MONSTER: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, Copy, Clone)]
struct TileRotation {
    id: usize,
    flipped: bool,
    rotation: Rotation,
}

impl TileRotation {
    fn from_up_and_left(id: usize, up: usize, left: usize) -> Self {
        let (flipped, rotation): (bool, Rotation) = match (up, left) {
            (0, 3) => (false, Rotation::Up),
            (0, 1) => (true, Rotation::Up),
            (1, 0) => (false, Rotation::Right),
            (1, 2) => (true, Rotation::Right),
            (2, 1) => (false, Rotation::Down),
            (2, 3) => (true, Rotation::Down),
            (3, 2) => (false, Rotation::Left),
            (3, 0) => (true, Rotation::Left),
            _ => {
                println!("A bad position happened: {} {}", up, left);
                (false, Rotation::Up)
            } //Should not happen
        };

        Self {
            id,
            flipped,
            rotation,
        }
    }
}

#[derive(Debug)]
struct Tile {
    id: usize,
    pixels: [[bool; TILE_SIZE]; TILE_SIZE],
}

impl Tile {
    fn row_to_int(&self, j: usize) -> usize {
        let rev = self.pixels[j]
            .iter()
            .fold(0, |acc, &b| acc * 2 + b as usize);
        let norm = self.pixels[j]
            .iter()
            .rev()
            .fold(0, |acc, &b| acc * 2 + b as usize);
        max(rev, norm)
    }

    fn col_to_int(&self, i: usize) -> usize {
        let rev = self
            .pixels
            .iter()
            .map(|row| row[i])
            .fold(0, |acc, b| acc * 2 + b as usize);
        let norm = self
            .pixels
            .iter()
            .map(|row| row[i])
            .rev()
            .fold(0, |acc, b| acc * 2 + b as usize);
        max(rev, norm)
    }

    fn pixel_at(&self, rot: &TileRotation, col: usize, row: usize) -> bool {
        let (x, y) = (col + 1, row + 1);
        let size = TILE_SIZE - 1;
        let (x, y) = if rot.flipped { (size - x, y) } else { (x, y) };
        let (x, y) = match rot.rotation {
            Rotation::Up => (x, y),
            Rotation::Right => (size - y, x),
            Rotation::Down => (size - x, size - y),
            Rotation::Left => (y, size - x),
        };
        self.pixels[y][x]
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let first: &str = lines.first().unwrap();
        let words: Vec<&str> = first.split(&[' ', ':']).collect();
        let id: usize = words[1].parse().unwrap();

        let mut pixels: [[bool; TILE_SIZE]; TILE_SIZE] = [[false; TILE_SIZE]; TILE_SIZE];
        for (j, row) in lines[1..].iter().enumerate() {
            for (i, pixel) in row.chars().enumerate() {
                if pixel == '#' {
                    pixels[j][i] = true;
                }
            }
        }

        Ok(Tile { id, pixels })
    }
}

#[derive(Debug)]
struct Monster {
    grid: Vec<Vec<bool>>,
}

impl Monster {
    fn size(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn all_orientations(&self) -> Vec<Vec<Vec<bool>>> {
        let mut all_orient: Vec<Vec<Vec<bool>>> = Vec::new();
        let mut current: Vec<Vec<bool>> = self.grid.clone();

        for _ in 0..4 {
            all_orient.push(current.clone());
            all_orient.push(Self::flip(&current));
            current = Self::rot(&current);
        }
        all_orient
    }

    fn flip(source: &[Vec<bool>]) -> Vec<Vec<bool>> {
        source
            .iter()
            .map(|line| line.iter().rev().cloned().collect())
            .collect()
    }

    fn rot(source: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let height: usize = source[0].len();
        let width: usize = source.len();
        let mut grid: Vec<Vec<bool>> = vec![vec![false; width]; height];

        for (j, row) in source.iter().enumerate() {
            for (i, &elem) in row.iter().enumerate() {
                grid[height - i - 1][j] = elem
            }
        }
        grid
    }
}

impl FromStr for Monster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<bool>> = s
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Ok(Self { grid })
    }
}

#[derive(Debug)]
struct Image {
    tiles: HashMap<usize, Tile>,
    all_edges: HashMap<usize, Vec<usize>>,
    edge_map: HashMap<usize, usize>,
    square: Vec<Vec<Option<TileRotation>>>,
    image: Vec<Vec<bool>>,
}

impl Image {
    fn corners_product(&self) -> usize {
        let size = self.square.len();
        let corners: Vec<(usize, usize)> =
            vec![(0, 0), (0, size - 1), (size - 1, 0), (size - 1, size - 1)];
        corners
            .into_iter()
            .map(|(col, row)| self.square[row][col].map(|tile| tile.id).unwrap_or(0))
            .product()
    }

    fn nb_monster(&self, monster: &[Vec<bool>]) -> usize {
        let height: usize = monster.len();
        let width: usize = monster[0].len();
        let mut nb = 0;

        let image_size = self.image.len();

        for y in 0..=(image_size - height) {
            for x in 0..=(image_size - width) {
                let is_monster: bool = monster.iter().enumerate().all(|(j, row)| {
                    row.iter()
                        .enumerate()
                        .all(|(i, &pixel)| !pixel || self.image[y + j][x + i])
                });
                if is_monster {
                    nb += 1
                }
            }
        }
        nb
    }

    fn water_roughness(&self) -> usize {
        self.image
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn monster_score(&self, monster: &str) -> usize {
        let monster: Monster = monster.parse().unwrap();
        let monster_size = monster.size();
        let max_monster: usize = monster
            .all_orientations()
            .into_iter()
            .find_map(|m| {
                let nb_monster = self.nb_monster(&m);
                if nb_monster > 0 {
                    Some(nb_monster)
                } else {
                    None
                }
            })
            .unwrap();
        self.water_roughness() - monster_size * max_monster
    }

    fn single_edges(&self, edges: &[usize]) -> Vec<usize> {
        edges
            .iter()
            .enumerate()
            .filter(|(_, edge)| self.edge_map.get(*edge).cloned().unwrap_or(0) == 1)
            .map(|(idx, _)| idx)
            .collect()
    }

    fn pixel_at(&self, col: usize, row: usize) -> bool {
        let tile_size = TILE_SIZE - 2;
        let tile_row: usize = row / tile_size;
        let tile_col: usize = col / tile_size;
        let y: usize = row % tile_size;
        let x: usize = col % tile_size;

        let tile_rot: TileRotation = self.square[tile_row][tile_col].unwrap();
        self.tiles
            .get(&tile_rot.id)
            .map(|tile| tile.pixel_at(&tile_rot, x, y))
            .unwrap()
    }

    fn build_image(&mut self) {
        self.build_square();

        let mut image = self.image.clone();

        for (j, row) in image.iter_mut().enumerate() {
            for (i, pixel) in row.iter_mut().enumerate() {
                *pixel = self.pixel_at(i, j);
            }
        }
        self.image = image;
    }

    fn build_square(&mut self) {
        let size = self.square.len();
        //Build each line by comparing the edges to the UP and LEFT neighbour
        for row in 0..size {
            for col in 0..size {
                let (up_id, up_edge): (Option<usize>, Option<usize>) = if row == 0 {
                    (None, None)
                } else if let Some((id, edge)) = self.bottom_edge(col, row - 1) {
                    (Some(id), Some(edge))
                } else {
                    (None, None)
                };
                let (left_id, left_edge): (Option<usize>, Option<usize>) = if col == 0 {
                    (None, None)
                } else if let Some((id, edge)) = self.right_edge(col - 1, row) {
                    (Some(id), Some(edge))
                } else {
                    (None, None)
                };
                let tile_rotation: TileRotation = self
                    .all_edges
                    .iter()
                    //Remove the top and left tiles from the candidates
                    .filter(|(id, _)| **id != up_id.unwrap_or(0) && **id != left_id.unwrap_or(0))
                    //Find the tile matching both edges and build its TileRotation
                    .find_map(|(&id, edges)| match (up_edge, left_edge) {
                        //Default case (core pieces)
                        (Some(up), Some(left)) => {
                            match (
                                edges.iter().position(|&edge| edge == up),
                                edges.iter().position(|&edge| edge == left),
                            ) {
                                (Some(u), Some(l)) => {
                                    Some(TileRotation::from_up_and_left(id, u, l))
                                }
                                _ => None,
                            }
                        }
                        //Looking for first column pieces
                        (Some(up), None) => {
                            let singles: Vec<usize> = self.single_edges(edges);
                            match (edges.iter().position(|&edge| edge == up), singles.len()) {
                                (Some(u), 1..=2) => {
                                    let left: usize = singles
                                        .into_iter()
                                        .find(|idx| {
                                            let diff: usize = idx.abs_diff(u);
                                            diff == 1 || diff == 3
                                        })
                                        .unwrap();
                                    Some(TileRotation::from_up_and_left(id, u, left))
                                }
                                _ => None,
                            }
                        }
                        //Looking for first row pieces
                        (None, Some(left)) => {
                            let singles: Vec<usize> = self.single_edges(edges);
                            match (edges.iter().position(|&edge| edge == left), singles.len()) {
                                (Some(l), 1..=2) => {
                                    let up: usize = singles
                                        .into_iter()
                                        .find(|idx| {
                                            let diff: usize = idx.abs_diff(l);
                                            diff == 1 || diff == 3
                                        })
                                        .unwrap();
                                    Some(TileRotation::from_up_and_left(id, up, l))
                                }
                                _ => None,
                            }
                        }
                        //Looking for the first corner
                        (None, None) => {
                            let singles: Vec<usize> = self.single_edges(edges);
                            if singles.len() == 2 {
                                Some(TileRotation::from_up_and_left(id, singles[0], singles[1]))
                            } else {
                                None
                            }
                        }
                    })
                    .unwrap();
                //Insert into square
                self.square[row][col] = Some(tile_rotation);
            }
        }
    }

    fn get_edge_with_id(&self, rot: &TileRotation, edge: usize) -> usize {
        let edges: &[usize] = self.all_edges.get(&rot.id).unwrap();
        let final_edge: usize = if rot.flipped && edge % 2 == 1 {
            4 - edge
        } else {
            edge
        };
        let idx: usize = (final_edge + rot.rotation as usize) % 4;
        edges[idx]
    }

    fn bottom_edge(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        self.square[y][x].map(|rot| (rot.id, self.get_edge_with_id(&rot, 2)))
    }

    fn right_edge(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        self.square[y][x].map(|rot| (rot.id, self.get_edge_with_id(&rot, 1)))
    }

    #[allow(dead_code)]
    fn print_image(&self) {
        for row in self.square.iter() {
            for rot in row.iter() {
                let rot = rot.unwrap();
                print!("{:?}\t", rot);
            }
            println!();
        }
        for (j, row) in self.image.iter().enumerate() {
            if j % 8 == 0 {
                println!();
            }
            for (i, &pixel) in row.iter().enumerate() {
                if i % 8 == 0 {
                    print!(" ");
                }
                let c: char = if pixel { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<&str> = util::split_blocks(s);
        let tiles: HashMap<usize, Tile> = blocks
            .iter()
            .map(|block| block.parse::<Tile>().unwrap())
            .map(|tile| (tile.id, tile))
            .collect();

        //Compute edges hash
        let all_edges: HashMap<usize, Vec<usize>> = tiles
            .iter()
            .map(|(&id, tile)| {
                (
                    id,
                    vec![
                        tile.row_to_int(0),             //Up
                        tile.col_to_int(TILE_SIZE - 1), //Right
                        tile.row_to_int(TILE_SIZE - 1), //Down
                        tile.col_to_int(0),             // Left
                    ],
                )
            })
            .collect();
        //Pair the edges
        let mut edge_map: HashMap<usize, usize> = HashMap::new();
        all_edges.iter().for_each(|(_, edges)| {
            edges.iter().for_each(|&edge| {
                *edge_map.entry(edge).or_insert(0) += 1;
            });
        });
        //Build empty square
        let square_size: usize = (tiles.len() as f64).sqrt() as usize;
        let square: Vec<Vec<Option<TileRotation>>> = vec![vec![None; square_size]; square_size];

        //Build empty image
        let image_size = square_size * (TILE_SIZE - 2);
        let image: Vec<Vec<bool>> = vec![vec![false; image_size]; image_size];
        Ok(Image {
            tiles,
            all_edges,
            edge_map,
            square,
            image,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_20.txt").expect("Cannot open input file");
    let mut image: Image = s.parse().unwrap();
    image.build_image();
    println!(
        "Part1: The product of the corners IDs is {}",
        image.corners_product()
    );
    println!(
        "The Water roughness surrounding the sea monsters is: {}",
        image.monster_score(MONSTER)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_1() {
        let mut image: Image = INPUT.parse().unwrap();
        image.build_image();
        image.print_image();
        assert_eq!(image.corners_product(), 20_899_048_083_289);
        assert_eq!(image.monster_score(MONSTER), 273);
    }
}
