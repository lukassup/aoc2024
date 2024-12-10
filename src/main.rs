#![feature(iter_array_chunks)]
use std::{io, time::Instant};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day9;
mod day10;

fn main() -> io::Result<()> {
    let now = Instant::now();
    day1::part1("input/day1.txt").map(|n| println!("day1p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day1::part2("input/day1.txt").map(|n| println!("day1p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day2::part1("input/day2.txt").map(|n| println!("day2p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day2::part2("input/day2.txt").map(|n| println!("day2p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day3::part1("input/day3.txt").map(|n| println!("day3p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day3::part2("input/day3.txt").map(|n| println!("day3p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day4::part1("input/day4.txt").map(|n| println!("day4p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day4::part2("input/day4.txt").map(|n| println!("day4p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day5::part1("input/day5.txt").map(|n| println!("day5p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day5::part2("input/day5.txt").map(|n| println!("day5p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day9::part1("input/day9.txt").map(|n| println!("day9p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day9::part2("input/day9.txt").map(|n| println!("day9p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day10::part1("input/day10.txt").map(|n| println!("day10p1: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    let now = Instant::now();
    day10::part2("input/day10.txt").map(|n| println!("day10p2: {n}"))?;
    eprintln!("⏱️ {:?}", now.elapsed());

    Ok(())
}
