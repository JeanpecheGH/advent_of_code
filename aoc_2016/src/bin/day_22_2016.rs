#[derive(Debug, Copy, Clone)]
struct Server {
    size: usize,
    used: usize,
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_22.txt").expect("Cannot open input file");

    let pos_servers: Vec<((usize, usize), Server)> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            let coords: Vec<&str> = words[0].split('-').collect();
            let x: usize = coords[1].strip_prefix('x').unwrap().parse().unwrap();
            let y: usize = coords[2].strip_prefix('y').unwrap().parse().unwrap();
            let size: usize = words[1].strip_suffix('T').unwrap().parse().unwrap();
            let used: usize = words[2].strip_suffix('T').unwrap().parse().unwrap();
            ((x, y), Server { size, used })
        })
        .collect();

    //Part 1: Find the only empty server, and check against its size
    let servers: Vec<Server> = pos_servers.iter().map(|trio| trio.1).collect();
    let empty_size: usize = servers.iter().filter(|s| s.used == 0).map(|s| s.size).sum();

    let can_be_moved = servers
        .iter()
        .filter(|s| s.used <= empty_size && s.used > 0)
        .count();
    println!(
        "Part1: {} server can be moved to the only empty server",
        can_be_moved
    );

    // Part 2
    //
    // There is a "wall" of big server which we cannot pass through.
    //
    // First we have to move the empty server left of this wall.
    //
    // Then go up to first row.
    //
    // Then go right to x36-y0 server.
    //
    // Finally, we come back to the x0-y0 server (5 moves by move of the server)

    let left_wall = pos_servers
        .iter()
        .filter_map(|((x, _), s)| if s.used > empty_size { Some(x) } else { None })
        .min()
        .unwrap()
        - 1;

    let (empty_x, empty_y) = pos_servers
        .iter()
        .find_map(|&((x, y), s)| if s.used == 0 { Some((x, y)) } else { None })
        .unwrap();
    let res = (empty_x - left_wall) + empty_y + (36 - left_wall) + (5 * 35);
    println!(
        "Part2: We can access the data of target server in {} moves",
        res
    );
}
