use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};
use std::{fs::File, io, path::Path};

type Coord = usize;
type Point = (Coord, Coord);
type Elevation = u32;

const NEIGH_OFFSET: [(isize, isize); 4] = [
    (0, -1), // up
    (1, 0),  // right
    (0, 1),  // down
    (-1, 0), // left
];

fn iter_neigh(grid: &[Vec<Elevation>], start: Point) -> impl Iterator<Item = Point> {
    let (x_start, y_start) = start;
    let x_max = grid[0].len();
    let y_max = grid.len();
    let val_start = &grid[y_start][x_start];
    NEIGH_OFFSET.iter().filter_map(move |&(dx, dy)| {
        let x = x_start.checked_add_signed(dx)?;
        let y = y_start.checked_add_signed(dy)?;
        if x >= x_max || y >= y_max {
            return None;
        }
        let val_neigh = grid.get(y).and_then(|row| row.get(x))?;
        if val_neigh.saturating_sub(*val_start) != 1 {
            return None;
        }
        Some((x, y))
    })
}

fn bfs(map: &[Vec<Elevation>], start: Point) -> HashSet<Point> {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new(); // !
    queue.push_back(start);
    while let Some(point) = queue.pop_front() {
        visited.insert(point);
        for neigh_point in iter_neigh(map, point) {
            if visited.contains(&neigh_point) {
                continue;
            }
            queue.push_back(neigh_point);
        }
    }
    visited
}

fn bfs_vec(map: &[Vec<Elevation>], start: Point) -> Vec<Point> {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut visited: Vec<Point> = Vec::new(); // !
    queue.push_back(start);
    while let Some(point) = queue.pop_front() {
        visited.push(point);
        for neigh_point in iter_neigh(map, point) {
            if visited.contains(&neigh_point) {
                continue;
            }
            queue.push_back(neigh_point);
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

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<usize> {
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
        let visited = bfs_vec(&map, start);
        let score = visited.iter().filter(|&&(x, y)| map[y][x] == 9).count();
        result += score;
    }
    Ok(result)
}
