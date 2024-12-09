use std::io::{BufRead, BufReader};
use std::{fs::File, io, path::Path};

// aoc inputs are ascii - use bytes as iterating over them is simpler
const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
const SAMX: [u8; 4] = [b'S', b'A', b'M', b'X'];

pub fn part1<T: AsRef<Path>>(path: T) -> io::Result<i32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut result = 0;
    let lines: Vec<Vec<u8>> = buf.lines().map_while(Result::ok).map(String::into_bytes).collect();

    // horizontal lines (window: 1x4)
    for line in lines.iter() {
        for hline in line.windows(4) {
            if hline == XMAS || hline == SAMX {
                result += 1;
            }
        }
    }

    for lines in lines.windows(4) {
        let l0 = lines[0].iter();
        let l1 = lines[1].iter();
        let l2 = lines[2].iter();
        let l3 = lines[3].iter();
        // vertical lines (window: 4x1)
        for (((&ch0, &ch1), &ch2), &ch3) in l0.zip(l1).zip(l2).zip(l3) {
            let vline = [ch0, ch1, ch2, ch3];
            if vline == XMAS || vline == SAMX {
                result += 1;
            }
        }

        // diagonal lines (window: 4x4)
        let l0 = lines[0].windows(4);
        let l1 = lines[1].windows(4);
        let l2 = lines[2].windows(4);
        let l3 = lines[3].windows(4);
        for (((l0, l1), l2), l3) in l0.zip(l1).zip(l2).zip(l3) {
            let dline1 = [l0[0], l1[1], l2[2], l3[3]];
            if dline1 == XMAS || dline1 == SAMX {
                result += 1;
            }
            let dline2 = [l0[3], l1[2], l2[1], l3[0]];
            if dline2 == XMAS || dline2 == SAMX {
                result += 1;
            }
        }
    }
    Ok(result)
}

pub fn part2<T: AsRef<Path>>(path: T) -> io::Result<i32> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let mut result = 0;
    let lines: Vec<Vec<u8>> = buf.lines().map_while(Result::ok).map(String::into_bytes).collect();
    // diagonal lines (window: 3x3)
    for lines in lines.windows(3) {
        let l0 = lines[0].windows(3);
        let l1 = lines[1].windows(3);
        let l2 = lines[2].windows(3);
        for ((w0, w1), w2) in l0.zip(l1).zip(l2) {
            match [w0, w1, w2] {
                [
                    [b'M',    _, b'M'],
                    [   _, b'A',    _],
                    [b'S',    _, b'S'],
                ] |
                [
                    [b'M',    _, b'S'],
                    [   _, b'A',    _],
                    [b'M',    _, b'S'],
                ] |
                [
                    [b'S',    _, b'S'],
                    [   _, b'A',    _],
                    [b'M',    _, b'M'],
                ] |
                [
                    [b'S',    _, b'M'],
                    [   _, b'A',    _],
                    [b'S',    _, b'M'],
                ] => result += 1,
                _ => {},
            }
        }
    }
    Ok(result)
}
