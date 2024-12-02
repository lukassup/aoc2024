use std::{fs::File, io, path::Path};
use std::io::{BufRead, BufReader};


pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<()> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();
    for line in buf.lines().map_while(Result::ok) {
        let Some((val1, val2)) = line.split_once("   ") else {
            continue;
        };
        let val1: i64 = val1.parse().map_err(|_| io::ErrorKind::InvalidData)?;
        let val2: i64 = val2.parse().map_err(|_| io::ErrorKind::InvalidData)?;
        list1.push(val1);
        list2.push(val2);
    }
    list1.sort();
    list2.sort();
    let result: u64 = list1.iter().zip(list2.iter()).map(|(&v1, &v2)| v1.abs_diff(v2)).sum();
    println!("part1: {result}");
    Ok(())
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<()> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();
    for line in buf.lines().map_while(Result::ok) {
        let Some((val1, val2)) = line.split_once("   ") else {
            continue;
        };
        let val1: i64 = val1.parse().map_err(|_| io::ErrorKind::InvalidData)?;
        let val2: i64 = val2.parse().map_err(|_| io::ErrorKind::InvalidData)?;
        list1.push(val1);
        list2.push(val2);
    }
    let mut result: i64 = 0;
    for v1 in list1.iter() {
        result += v1 * list2.iter().filter(|&v2| v2 == v1).count() as i64;
    }
    println!("part2: {result}");
    Ok(())
}