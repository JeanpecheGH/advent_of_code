use std::str::FromStr;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

struct Image {
    layers: Vec<Vec<u8>>,
}

impl Image {
    fn fewest_zero_score(&self) -> usize {
        let layer: usize = (0..self.layers.len())
            .map(|i| (i, self.nb_n_in_layer(i, 0)))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;
        self.layer_score(layer)
    }

    fn nb_n_in_layer(&self, layer: usize, n: u8) -> usize {
        self.layers[layer].iter().filter(|p| **p == n).count()
    }

    fn layer_score(&self, layer: usize) -> usize {
        self.nb_n_in_layer(layer, 1) * self.nb_n_in_layer(layer, 2)
    }

    fn apply_layers(&self) -> Vec<u8> {
        let mut final_image: Vec<u8> = vec![2; WIDTH * HEIGHT];
        for layer in self.layers.iter() {
            for (i, &p) in layer.iter().enumerate() {
                if final_image[i] == 2 {
                    final_image[i] = p;
                }
            }
        }
        final_image
    }

    fn print_image(&self) {
        let image: Vec<u8> = self.apply_layers();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let c: &str = if image[y * WIDTH + x] == 1 {
                    "██"
                } else {
                    "  "
                };
                print!("{c}");
            }
            println!();
        }
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u8> = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let layers: Vec<Vec<u8>> = numbers
            .chunks(WIDTH * HEIGHT)
            .map(|chunk| chunk.to_vec())
            .collect();

        Ok(Image { layers })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_08.txt").expect("Cannot open input file");
    let image: Image = s.lines().next().unwrap().parse().unwrap();
    println!(
        "Part1: The layer with the fewest 0 has a score of {}",
        image.fewest_zero_score()
    );
    println!("Part2:");
    image.print_image();
    println!("Computing time: {:?}", now.elapsed());
}
