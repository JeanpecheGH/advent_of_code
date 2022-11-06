#[derive(Copy, Clone)]
struct LetterCounter {
    letters: [u8; 26],
}

impl LetterCounter {
    fn new(low: bool) -> Self {
        if low {
            LetterCounter { letters: [0; 26] }
        } else {
            LetterCounter {
                letters: [u8::MAX; 26],
            }
        }
    }

    fn add(&mut self, c: char) {
        let index = c as usize - 'a' as usize;
        if self.letters[index] == u8::MAX {
            self.letters[index] = 0;
        }
        self.letters[index] += 1;
    }

    fn ret_char(&self, low: bool) -> char {
        let mut index = 0;
        let mut limit = if low { 0 } else { u8::MAX };
        self.letters.iter().enumerate().for_each(|(i, &n)| {
            if low {
                if n > limit {
                    limit = n;
                    index = i;
                }
            } else if n < limit {
                limit = n;
                index = i;
            }
        });

        char::from_u32(index as u32 + 'a' as u32).unwrap()
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_06.txt").expect("Cannot open input file");

    let bad_words: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let mut counters: [LetterCounter; 8] = [LetterCounter::new(true); 8];
    bad_words
        .iter()
        .for_each(|w| w.chars().enumerate().for_each(|(i, c)| counters[i].add(c)));

    let first_pwd: String = counters.iter().map(|cnt| cnt.ret_char(true)).collect();
    println!("Part1: The password is {}", first_pwd);

    counters = [LetterCounter::new(false); 8];
    bad_words
        .iter()
        .for_each(|w| w.chars().enumerate().for_each(|(i, c)| counters[i].add(c)));

    let second_pwd: String = counters.iter().map(|cnt| cnt.ret_char(false)).collect();
    println!("Part1: The password is {}", second_pwd);
}
