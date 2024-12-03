use std::io::{BufRead, BufReader};
use std::{fs::File, io, path::Path};

use regex::Regex;

pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<i32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for line in buf.lines().map_while(Result::ok) {
        for (_, [a, b ]) in re_mul.captures_iter(&line).map(|c| c.extract()) {
            let a: i32 = a.parse().map_err(|_| io::ErrorKind::InvalidData)?;
            let b: i32 = b.parse().map_err(|_| io::ErrorKind::InvalidData)?;
            result += a * b;
        }
    }
    Ok(result)
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<i32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let re = Regex::new(r"(do\(()()\)|don't\(()()\)|mul\((\d+),(\d+)\))").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for line in buf.lines().map_while(Result::ok) {
        for cap in re.captures_iter(&line) {
            match cap.extract().1 {
                ["don't()", _, _] => enabled = false,
                ["do()", _, _] => enabled = true,
                [_, a, b] => if enabled {
                    let a: i32 = a.parse().map_err(|_| io::ErrorKind::InvalidData)?;
                    let b: i32 = b.parse().map_err(|_| io::ErrorKind::InvalidData)?;
                    result += a * b;
                },
            }
        }
    }
    Ok(result)
}
