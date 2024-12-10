use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};
use std::{fs::File, io, path::Path};

type Coord = usize;
type Point = (Coord, Coord);
type Elevation = u32;

const NEIGH_OFFSET: [(isize, isize); 4] = [
    (0, -1), // up
    (1, 0), // right
    (0, 1), // down
    (-1, 0), // left
];

fn iter_neigh(grid: &[Vec<Elevation>], point: Point) -> impl Iterator<Item = Point> {
    let x_o = point.0;
    let y_o = point.1;
    let x_max = grid[0].len();
    let y_max = grid.len();
    let start_el = &grid[y_o][x_o];
    NEIGH_OFFSET.iter().filter_map(move |&(dx, dy)| {
        let x = x_o.checked_add_signed(dx)?;
        let y = y_o.checked_add_signed(dy)?;
        if x >= x_max || y >= y_max {
            return None;
        }
        let neigh_el = grid.get(y).and_then(|row| row.get(x))?;
        if neigh_el - *start_el != 1 {
            return None;
        }
        Some((x, y))
    })
}

fn bfs(map: &[Vec<Elevation>], start: Point) -> HashSet<Point> {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front()  {
        visited.insert(pos);
        for (x,y) in iter_neigh(map, pos) {
            if visited.contains(&(x,y)) {
                continue;
            }
            queue.push_back((x,y));
        }
    }
    visited
}

pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<usize> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let map: Vec<Vec<Elevation>> = buf
        .lines()
        .map_while(Result::ok)
        .map(|s| s.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut trailheads: Vec<Point> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, &elevation) in line.iter().enumerate() {
            if elevation == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let mut result: usize = 0;
    for start in trailheads {
        let visited = bfs(&map, start);
        let score = visited.iter().filter(|&&(x, y)| map[y][x] == 9).count();
        result += score;
    }
    Ok(result)
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<u64> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let _map: Vec<Vec<u32>> = buf
        .lines()
        .map_while(Result::ok)
        .map(|s| s.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();
    todo!();
    let result: u64 = 0;
    Ok(result)
}
