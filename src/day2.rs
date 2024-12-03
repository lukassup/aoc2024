use std::cmp::Reverse;
use std::io::{BufRead, BufReader};
use std::{fs::File, io, path::Path};

pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<i32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut result = 0;
    for line in buf.lines().map_while(Result::ok) {
        let nums: Vec<i32> = line.split(" ").flat_map(|s_num| s_num.parse()).collect();
        if !nums
            .windows(2)
            .all(|w| (w[0] - w[1]).abs() >= 1 && (w[0] - w[1]).abs() <= 3)
        {
            continue;
        }
        if !(nums.is_sorted() || nums.is_sorted_by_key(Reverse)) {
            continue;
        }
        result += 1;
    }
    Ok(result)
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<i32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut result = 0;
    for line in buf.lines().map_while(Result::ok) {
        let nums: Vec<i32> = line.split(" ").flat_map(|s_num| s_num.parse()).collect();
        // keep original & add permutations by dropping any 1 element
        let mut permutations: Vec<Vec<i32>> = vec![nums.clone()];
        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);
            permutations.push(nums);
        }
        // reject if none of permutations match
        if !permutations.iter().any(|nums| {
            nums.windows(2)
                .all(|w| (w[0] - w[1]).abs() >= 1 && (w[0] - w[1]).abs() <= 3)
                && (nums.is_sorted() || nums.is_sorted_by_key(Reverse))
        }) {
            continue;
        }
        result += 1;
    }
    Ok(result)
}
