use std::collections::VecDeque;

fn main() {
    let input: usize = 3017957;

    let now = std::time::Instant::now();
    let mut circle = VecDeque::from_iter(1..=input);
    while circle.len() > 1 {
        circle.rotate_left(1);
        circle.pop_front();
    }
    println!(
        "Part1: The winning elf is n° {}, found in {:?}",
        circle.pop_front().unwrap(),
        now.elapsed()
    );

    //Split the data in 2 VecDeque since we always remove elements in the middle
    let now = std::time::Instant::now();
    let mut front_circle = VecDeque::from_iter(1..=input.div_ceil(2));
    let mut back_circle = VecDeque::from_iter(input.div_ceil(2) + 1..=input);

    while (front_circle.len() + back_circle.len()) > 1 {
        if front_circle.len() > back_circle.len() {
            front_circle.pop_back();
        } else {
            back_circle.pop_front();
        }
        match back_circle.pop_front() {
            None => break,
            Some(v) => {
                front_circle.push_back(v);
                back_circle.push_back(front_circle.pop_front().unwrap());
            }
        }
    }
    println!(
        "Part2: The winning elf is n° {}, found in {:?}",
        front_circle.pop_front().unwrap(),
        now.elapsed()
    );
}
