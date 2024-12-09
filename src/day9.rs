use std::io::{BufRead, BufReader};
use std::{iter, mem};
use std::{fs::File, io, path::Path};

pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<u64> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut lines = buf.lines().map_while(Result::ok);
    let line = lines.next().ok_or(io::ErrorKind::InvalidData)?;

    let mut disk_map: Vec<Option<usize>> = Vec::new();
    line.chars().enumerate().for_each(|(fid, fsize)| {
        let fsize = fsize.to_digit(10).unwrap();
        let sector = if fid % 2 == 0 { Some(fid / 2) } else { None };
        for _ in 0..fsize {
            disk_map.push(sector);
        }
    });

    for i in (0..disk_map.len()).rev() {
        if disk_map[i].is_none() {
            continue;
        }
        for j in 0..i {
            if disk_map[j].is_none() {
                disk_map.swap(i, j);
                break;
            }
        }
    }

    let result: u64 = disk_map.iter().enumerate().fold(0, |acc, (i, &v)| {
        if let Some(v) = v {
            return acc + i as u64 * v as u64;
        }
        acc
    });
    Ok(result)
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<u64> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut lines = buf.lines().map_while(Result::ok);
    let line = lines.next().ok_or(io::ErrorKind::InvalidData)?;

    let mut disk_map: Vec<Vec<Option<usize>>> = Vec::new();
    line.chars().enumerate().for_each(|(fid, fsize)| {
        let fsize: usize = fsize.to_digit(10).unwrap() as usize;
        let block = if fid % 2 == 0 { Some(fid / 2) } else { None };
        disk_map.push(iter::repeat_n(block, fsize).collect());
        disk_map.push(Vec::new());
    });

    for i in (0..disk_map.len()).rev() {
        if disk_map[i].iter().all(Option::is_none) {
            continue;
        }
        for j in 0..i {
            if disk_map[j].iter().all(Option::is_none) {
                let rlen = disk_map[i].len();
                let llen = disk_map[j].len();
                #[allow(clippy::comparison_chain)]
                if llen == rlen {
                    disk_map.swap(i, j);
                    break;
                } else if llen > rlen {
                    let mut left = disk_map[j].clone();
                    let (left, rem) = left.split_at_mut(rlen);
                    let mut right = disk_map[i].clone();
                    left.swap_with_slice(&mut right);
                    let _ = mem::replace(&mut disk_map[i], right);
                    let _ = mem::replace(&mut disk_map[j], left.to_vec());
                    let _ = mem::replace(&mut disk_map[j+1], rem.to_vec());
                    break;
                }
            }
        }
    }

    let result: u64 = disk_map.iter().flatten().enumerate().fold(0, |acc, (i, &v)| {
        if let Some(v) = v {
            return acc + i as u64 * v as u64;
        }
        acc
    });
    Ok(result)
}
